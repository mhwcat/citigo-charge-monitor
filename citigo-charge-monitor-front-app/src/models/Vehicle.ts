export default interface Vehicle {
  id: string;
  vin: string;
  soc: number;
  targetSoc: number;
  lastUpdateTime: string;
}