from pydantic import BaseModel, validator
from typing import Optional, List
from humps import camelize
import datetime

VIN_LEN = 17

def to_camel(string):
    return camelize(string)

class CamelModel(BaseModel):
    class Config:
        alias_generator = to_camel
        allow_population_by_field_name = True


class LoginRequest(CamelModel):
    username: str
    password: str


class UserSession(CamelModel):
    id: str


class CreateVehicle(CamelModel):
    vin: str
    target_soc: int

    @validator('vin')
    def vin_validator(cls, v):
        if len(v) != VIN_LEN:
            raise ValueError('Wrong VIN value')
        return v

class UpdateVehicle(CamelModel):
    id: str
    soc: Optional[int]
    target_soc: Optional[int]   


class Vehicle(CamelModel):
    id: str
    soc: Optional[int]
    target_soc: int
    last_update_time: datetime.datetime
    vin: str

    @validator('vin')
    def vin_validator(cls, v):
        if len(v) != VIN_LEN:
            raise ValueError('Wrong VIN value')
        return v


class ChargeSession(CamelModel):
    id: str
    start_time: datetime.datetime
    stop_time: Optional[datetime.datetime]
    start_soc: int
    current_soc: int
    stop_soc: Optional[int]
    vehicle_id: str
    last_update_time: Optional[datetime.datetime]


class UpdateChargeSession(CamelModel):
    vehicle_id: str
    soc: int
    stop: bool

    @validator('soc')
    def soc_validator(cls, v):
        if v < 0 or v > 100:
            raise ValueError('Wrong SOC value')
        return v


class SkodaconnectChargeStatus(BaseModel):
    is_charging: bool
    soc: int        


class VehicleStatus(CamelModel):
    is_charging: bool
    vehicle: Vehicle
    current_charge_session: Optional[ChargeSession]    