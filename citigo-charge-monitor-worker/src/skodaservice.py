import logging
import schemas
import aiohttp
import skodaconnect
import os

CITIGO_SKODACONNECT_USERNAME = os.environ.get('CITIGO_SKODACONNECT_USERNAME')
CITIGO_SKODACONNECT_PASSWORD = os.environ.get('CITIGO_SKODACONNECT_PASSWORD')
assert CITIGO_SKODACONNECT_USERNAME is not None
assert CITIGO_SKODACONNECT_PASSWORD is not None

async def check_charging_status(vin: str)-> schemas.SkodaconnectChargeStatus:
    logging.info('Checking charging status in Skoda Connect...')

    cs = None
    async with aiohttp.ClientSession(headers={'Connection': 'keep-alive'}) as session:
        connection = skodaconnect.Connection(session, CITIGO_SKODACONNECT_USERNAME, CITIGO_SKODACONNECT_PASSWORD, True)
        logging.info("Attempting to login to the Skoda Connect service")
        if await connection.doLogin():
            await connection.get_vehicles()
            for vehicle in connection.vehicles:
                if vehicle.vin == vin:
                    cs = schemas.SkodaconnectChargeStatus(is_charging=vehicle.charging == 1, soc=vehicle.battery_level)
            await connection.logout()
    return cs


async def stop_charging(vin: str):
    logging.info('Stopping charging in Skoda Connect...')

    result = False
    async with aiohttp.ClientSession(headers={'Connection': 'keep-alive'}) as session:
        connection = skodaconnect.Connection(session, CITIGO_SKODACONNECT_USERNAME, CITIGO_SKODACONNECT_PASSWORD, True)
        if await connection.doLogin():
            await connection.get_vehicles()
            for vehicle in connection.vehicles:
                if vehicle.vin == vin:
                    result = await vehicle.set_charger(action='stop')
            await connection.logout()  
    
    return result    