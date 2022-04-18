<template>
<article class="panel">
  <p class="panel-heading">Status</p>
  <div class="panel-block">
    <div class="columns has-text-centered is-mobile pt-2 pb-2" style="width: 100%;">
      <div class="column">
        <span class="subtitle is-5" :class="colorForSoc">{{ vsStore.vehicleStatus.vehicle.soc }}%</span>
      </div>
      <div class="column">
        <span class="subtitle is-5 icon-text">
          <span class="has-text-info-dark" v-if="vsStore.vehicleStatus.isCharging">
            <span class="icon" style="vertical-align: middle;">
              <font-awesome-icon :icon="['fas', 'plug-circle-bolt']" />
            </span>
            <span>Charging</span>
          </span>
          <span v-if="!vsStore.vehicleStatus.isCharging">
            <span class="icon" style="vertical-align: middle;">
              <font-awesome-icon :icon="['fas', 'plug-circle-xmark']" />
            </span>
            <span>Not charging</span>
          </span>
        </span>
      </div>
    </div>
  </div>  
</article> 
</template>

<script lang="ts">
import { defineComponent } from "vue";
import vsStore from "@/stores/vehicle-status";

export default defineComponent({
  name: "vehicle-status-comp",
  data() {
    return {
      vsStore
    };
  },
  computed: {
    colorForSoc(): string {
      const soc = vsStore.vehicleStatus.vehicle.soc;

      if (soc <= 100 && soc >= 70) {
        return "has-text-success-dark";
      } else if (soc < 70 && soc >= 20) {
        return "has-text-warning-dark";
      } else {
        return "has-text-danger-dark";
      }
    }
  },
  mounted() {
    //
  },
});
</script>