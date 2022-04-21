<template>
<div>
    <nav class="navbar" role="navigation" aria-label="main navigation" style="width: 100%;">
      <div class="navbar-brand">
        <a class="navbar-item" href="#">
          <h1 class="title is-5">Citigo Charge Monitor</h1>
        </a>
      </div>
      <div class="navbar-end">
        <div class="navbar-item">
          <div class="dropdown" :class="{ 'is-active': vehicleSelectionOpen }">
            <div class="dropdown-trigger" @click="vehicleSelectionOpen = !vehicleSelectionOpen">
              <button class="button" aria-haspopup="true" aria-controls="dropdown-menu">
                <span>{{ store.getters.vehicleVin || "Select vehicle" }}</span>
                <span class="icon is-small">
                  <font-awesome-icon :icon="['fas', 'car']" />
                </span>
              </button>
            </div>
            <div class="dropdown-menu" id="dropdown-menu" role="menu">
              <div class="dropdown-content" v-for="vehicle in vehicles" :key="vehicle.id">
                <div class="dropdown-item">
                  <a href="#" @click="handleVehicleSelection(vehicle)">{{ vehicle.vin }}</a>
                </div>                 
              </div>
            </div>
          </div>          
        </div>        
        <div class="navbar-item">
          <div class="buttons">             
            <a class="button is-light" @click="handleLogout">
              <span class="icon is-small">
                <font-awesome-icon :icon="['fas', 'right-from-bracket']" />
              </span> 
            </a>
          </div>
        </div>
      </div>
    </nav>
    <VehicleStatusComp v-if="store.getters.vehicleId"/>
    <TargetSocSetting v-if="store.getters.vehicleId" />
    <CurrentChargeSession v-if="store.getters.vehicleId && vsStore.vehicleStatus.isCharging"/>  
    <ChargeSessions v-if="store.getters.vehicleId"/>       
</div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ChargeSessions from "@/components/ChargeSessions.vue";
import VehicleStatusComp from "@/components/VehicleStatusComp.vue";
import TargetSocSetting from "@/components/TargetSocSetting.vue";
import CurrentChargeSession from "@/components/CurrentChargeSession.vue";
import ApiService from "@/services/ApiService";
import vsStore from "@/stores/vehicle-status";
import { useToast } from "vue-toastification";
import { useStore } from "@/stores/store";
import { useRouter } from "vue-router";
import Vehicle from "@/models/Vehicle";

export default defineComponent({
  name: "main-component",
  setup() {
    const toast = useToast();
    const store = useStore();
    const router = useRouter();

    return { toast, store, router }
  },  
  data() {
    return {
      vsStore, 
      vehicles: [] as Vehicle[],
      vehicleSelectionOpen: false,
    }
  },
  components: {
    ChargeSessions, VehicleStatusComp, TargetSocSetting, CurrentChargeSession
  },
  methods: {
    fetchVehicles() {
      ApiService.getVehicles()
        .then((response: any) => {
          this.vehicles = response.data;
        });
    },
    fetchVehicleStatus() {
      if (this.store.getters.vehicleId && this.store.getters.userSessionId) {
        ApiService.getVehicleStatus(this.store.getters.vehicleId)
          .then((response: any) => {
            vsStore.vehicleStatus = response.data;
          });
      }
    },
    handleVehicleSelection(vehicle: Vehicle) {
      this.vehicleSelectionOpen = false;

      this.store.commit("saveVehicle", vehicle);
      this.fetchVehicleStatus();
    },
    handleLogout() {
      ApiService.logout()
        .finally(() => {
          // Clear user session either way, logout must always succeed
          this.store.commit("saveUserSession", null);
        });
    },
  },
  mounted() {
    this.fetchVehicles();
    this.fetchVehicleStatus();
  },
});
</script>
