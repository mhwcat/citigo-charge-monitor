<template>
<article class="panel">
  <p class="panel-heading">Target SOC</p>
  <div class="panel-block pt-2 pb-2">
    <input id="sliderWithValue" class="slider has-output is-fullwidth" min="0" max="100" step="5" 
      v-model="vsStore.vehicleStatus.vehicle.targetSoc" type="range" @change="targetSocChanged()">
    <output for="sliderWithValue">{{ vsStore.vehicleStatus.vehicle.targetSoc }}</output>
  </div>  
</article>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ApiService from "@/services/ApiService";
import vsStore from "@/stores/vehicle-status";
import { useToast } from "vue-toastification";
import { useStore } from "@/stores/store";

export default defineComponent({
  name: "target-soc-setting",
  setup() {
    const toast = useToast();
    const store = useStore();

    return { toast, store }
  },
  data() {
    return {
      vsStore
    };
  },
  methods: {
    targetSocChanged() {
      if (this.store.getters.vehicleId && this.store.getters.userSessionId) {
        ApiService.updateTargetSoc(this.store.getters.vehicleId, vsStore.vehicleStatus.vehicle.targetSoc)
          .then((response: any) => {
            this.toast.success("Target SOC updated", {
              timeout: 2000
            });
          });
      }
    }
  },
  mounted() {
    //
  },
});
</script>

<style scoped>
  @import "bulma-slider/dist/css/bulma-slider.min.css";
</style>