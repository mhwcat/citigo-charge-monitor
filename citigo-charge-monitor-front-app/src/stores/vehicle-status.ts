import { reactive } from "vue";
import VehicleStatus from "@/models/VehicleStatus";
import Vehicle from "@/models/Vehicle";

const vsStore = reactive({
  vehicleStatus: { vehicle: {} as Vehicle } as VehicleStatus
});

export default vsStore;