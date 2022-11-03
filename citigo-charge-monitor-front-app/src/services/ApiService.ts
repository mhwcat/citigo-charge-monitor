import apiClient from "@/http-common";

class ApiService {
  login(username: string, password: string): Promise<any> {
    return apiClient.post(`/auth/login`, { "username": username, "password": password });
  }

  logout(): Promise<any> {
    return apiClient.post(`/auth/logout`);
  }

  getVehicles(): Promise<any> {
    return apiClient.get(`/vehicle/all`);
  }

  getVehicleStatus(id: string): Promise<any> {
    return apiClient.get(`/vehicle/status?id=${id}`);
  }  

  getChargeSessions(vehicleId: string, index: number, pageSize: number): Promise<any> {
    return apiClient.get(`/chargeSession?vehicleId=${vehicleId}&index=${index}&pageSize=${pageSize}`);
  }

  updateTargetSoc(vehicleId: string, newTargetSoc: number): Promise<any> {
    return apiClient.put(`/vehicle`, { "id": vehicleId, "targetSoc": Number(newTargetSoc) });
  }  
}

export default new ApiService();