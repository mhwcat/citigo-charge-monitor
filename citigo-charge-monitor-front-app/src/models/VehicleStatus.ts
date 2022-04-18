import Vehicle from "@/models/Vehicle";
import { ChargeSession } from "@/models/ChargeSession";

export default interface VehicleStatus {
  vehicle: Vehicle;
  isCharging: boolean;
  currentChargeSession?: ChargeSession;
}