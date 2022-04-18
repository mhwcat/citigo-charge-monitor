import http from "@/http-common";

class ApiService {
  login(username: string, password: string): Promise<any> {
    return http.post(`/auth/login`, { "username": username, "password": password });
  }

  logout(): Promise<any> {
    return http.post(`/auth/logout`);
  }

  getVehicles(): Promise<any> {
    return http.get(`/vehicle/all`);
  }

  getVehicleStatus(id: string): Promise<any> {
    return http.get(`/vehicle/status?id=${id}`);
  }  

  getChargeSessions(vehicleId: string, index: number, pageSize: number): Promise<any> {
    return http.get(`/chargeSession?vehicleId=${vehicleId}&index=${index}&pageSize=${pageSize}`);
  }

  updateTargetSoc(vehicleId: string, newTargetSoc: number): Promise<any> {
    return http.put(`/vehicle`, { "id": vehicleId, "targetSoc": Number(newTargetSoc) });
  }  
}

export default new ApiService();