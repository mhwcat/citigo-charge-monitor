export interface ChargeSession {
  id: string;
  startSoc: number;
  stopSoc: number;
  startTime: string;
  stopTime: string;
}

export interface ChargeSessionsList {
  limit: number,
  offset: number,
  data: ChargeSession[]
}