import logging
import schemas
import api
from datetime import datetime

def login(username: str, password: str) -> schemas.UserSession:
    user_session = api.login(schemas.LoginRequest(**{'username': username, 'password': password}))

    return schemas.UserSession(**user_session.json())


def logout(session_id: str):
    api.logout(session_id)


def get_or_create_vehicle(vin: str, token: str) -> schemas.Vehicle:
    vehicle = api.get_vehicle_by_vin(vin, token)
    if vehicle.status_code == 404:
        logging.warning('Vehicle %s does not exist, creating', vin)
        vehicle_to_create = schemas.CreateVehicle(vin=vin, target_soc=80)
        vehicle = api.create_vehicle(vehicle=vehicle_to_create, token=token)
    return schemas.Vehicle(**vehicle.json())


def get_vehicle_status(id: str, token: str) -> schemas.VehicleStatus:
    vehicle_status = api.get_vehicle_status(id, token)

    return schemas.VehicleStatus(**vehicle_status.json())


def update_vehicle(vehicle_id: str, current_soc: int, token: str) -> schemas.Vehicle:
    response = api.update_vehicle(schemas.UpdateVehicle(id=vehicle_id, soc=current_soc), token=token)
        
    return schemas.Vehicle(**response.json())


def update_charge_session(vehicle_id: str, current_soc: int, stop_charging: bool, token: str) -> schemas.ChargeSession:
    charge_session = schemas.UpdateChargeSession(**{'vehicle_id': vehicle_id, 'soc': current_soc, 'stop': stop_charging})
    response = api.update_charge_session(charge_session=charge_session, token=token)
        
    return schemas.ChargeSession(**response.json())