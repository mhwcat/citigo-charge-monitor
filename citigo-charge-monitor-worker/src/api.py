import logging
import os
import requests as rq
from schemas import UpdateVehicle, CreateVehicle, LoginRequest, UpdateChargeSession

BASE_URL = os.environ.get('CITIGO_API_BASE_URL')
assert BASE_URL is not None

def login(request: LoginRequest):
    response = rq.post(BASE_URL + 'auth/login', data=request.json(), headers={'Content-type': 'application/json'})

    if response.status_code == 401:
        logging.error('Failed logging in to API, wrong username/password!')

    return response
    

def logout(token: str):
    return rq.post(BASE_URL + 'auth/logout', headers={'Authorization': 'Bearer ' + token})


def get_vehicles(token: str):
    return rq.get(BASE_URL + 'vehicle', auth=BearerAuth(token))


def get_vehicle(_id: str, token: str):
    return rq.get(BASE_URL + 'vehicle?id=' + _id, auth=BearerAuth(token))


def get_vehicle_by_vin(vin: str, token: str):
    return rq.get(BASE_URL + 'vehicle?vin=' + vin, auth=BearerAuth(token))


def create_vehicle(vehicle: CreateVehicle, token: str):
    return rq.post(BASE_URL + 'vehicle', data=vehicle.json(by_alias=True), headers={'Content-type': 'application/json'}, auth=BearerAuth(token))


def get_vehicle_status(id: str, token: str):
    return rq.get(BASE_URL + 'vehicle/status?id=' + id, auth=BearerAuth(token))


def update_vehicle(vehicle: UpdateVehicle, token: str):
    return rq.put(BASE_URL + 'vehicle', data=vehicle.json(by_alias=True), headers={'Content-type': 'application/json'}, auth=BearerAuth(token))


def update_charge_session(charge_session: UpdateChargeSession, token: str):
    return rq.post(BASE_URL + 'chargeSession', data=charge_session.json(by_alias=True), headers={'Content-type': 'application/json'}, auth=BearerAuth(token))    
       

class BearerAuth(rq.auth.AuthBase):
    def __init__(self, token):
        self.token = token

    def __call__(self, r):
        r.headers['Authorization'] = 'Bearer ' + self.token
        return r       