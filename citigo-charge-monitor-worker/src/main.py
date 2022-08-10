import logging
import asyncio
import os
import apiservice 
import skodaservice 

CITIGO_VEHICLE_VIN = os.environ.get('CITIGO_VEHICLE_VIN')
CITIGO_API_USERNAME = os.environ.get('CITIGO_API_USERNAME');
CITIGO_API_PASSWORD = os.environ.get('CITIGO_API_PASSWORD');
assert CITIGO_VEHICLE_VIN is not None
assert CITIGO_API_USERNAME is not None
assert CITIGO_API_PASSWORD is not None

# Skodaconnect query interval: 15 minutes when not charging, 5 minutes when charging
RUNTIME_INTERVAL_SECONDS = 900

async def repeat_task(func, *args, **kwargs):
    while True:
        try:        
            await func(*args, **kwargs)
        except Exception as e:
            logging.error('Failed executing task! %s', e)
        await asyncio.sleep(RUNTIME_INTERVAL_SECONDS)


async def main(): 
    logging.info('Starting citigo-charge-monitor-worker')
    
    auth_token = apiservice.login(CITIGO_API_USERNAME, CITIGO_API_PASSWORD)

    vehicle = apiservice.get_or_create_vehicle(CITIGO_VEHICLE_VIN, auth_token.id)
    logging.info('Vehicle: %s', vehicle)

    vehicle_status = apiservice.get_vehicle_status(vehicle.id, auth_token.id)
    logging.info('Vehicle status: %s', vehicle_status)

    charge_status = await skodaservice.check_charging_status(vehicle.vin)
    logging.info('Charge info from Skoda Connect: Charging: %s / SOC: %d', charge_status.is_charging, charge_status.soc)

    stop_charging = False
    charge_session = None
    global RUNTIME_INTERVAL_SECONDS

    if charge_status.is_charging:   
        RUNTIME_INTERVAL_SECONDS = 300
        if charge_status.soc >= vehicle.target_soc:
            logging.info('Vehicle current SOC %d%% exceeds target SOC %d%%, stopping charging...', charge_status.soc, vehicle.target_soc)

            if await skodaservice.stop_charging(vehicle.vin) == 'Success':
                stop_charging = True
        charge_session = apiservice.update_charge_session(vehicle.id, charge_status.soc, stop_charging, auth_token.id)
    else:
        RUNTIME_INTERVAL_SECONDS = 900        
        if vehicle_status.is_charging:
            stop_charging = True
            charge_session = apiservice.update_charge_session(vehicle.id, charge_status.soc, stop_charging, auth_token.id)

    logging.info('Created/updated charge session: %s', charge_session)

    # If charge session was not created/updated during execution, call separate endpoint to update current vehicle SOC
    if charge_session is None:
        apiservice.update_vehicle(vehicle.id, charge_status.soc, auth_token.id)
        logging.info('Updated vehicle SOC: ID: %s, SOC: %s', vehicle.id, charge_status.soc)

    apiservice.logout(auth_token.id)

if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO, format='%(asctime)s [%(levelname)s] %(message)s',
        handlers=[logging.StreamHandler()]) 

    loop = asyncio.new_event_loop()
    loop.create_task(repeat_task(main))
    loop.run_forever()