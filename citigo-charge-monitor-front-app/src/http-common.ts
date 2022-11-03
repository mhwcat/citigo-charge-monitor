import axios, { AxiosInstance } from "axios";

const apiClient: AxiosInstance = axios.create({
  baseURL: "/api",
  // Axios 1.x does not play well with headers here, let him resolve them 
  // headers: {
  //   "Content-type": "application/json",
  // },
});

export default apiClient;