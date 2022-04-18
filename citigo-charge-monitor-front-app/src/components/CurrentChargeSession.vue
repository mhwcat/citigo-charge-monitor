<template>
<article class="panel is-info">
  <p class="panel-heading">Current charge session</p>
  <div class="panel-block">
    <table class="table is-fullwidth">
      <thead>
          <th class="has-text-left">Started at</th>
          <th class="has-text-left">Start SOC</th>
          <th class="has-text-left">Current SOC</th>
      </thead>
      <tbody>
        <tr>
          <td>{{ formatDt(vsStore.vehicleStatus.currentChargeSession.startTime) }}</td>
          <td>{{ vsStore.vehicleStatus.currentChargeSession.startSoc }}%</td>
          <td>{{ vsStore.vehicleStatus.currentChargeSession.currentSoc }}%</td>
        </tr>
      </tbody>
    </table>
  </div>
</article>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import vsStore from "@/stores/vehicle-status";
import { formatDt } from "@/main";

export default defineComponent({
  name: "current-charge-session",
  setup() {
    return { formatDt };
  },
  data() {
    return { vsStore };
  },
});
</script>