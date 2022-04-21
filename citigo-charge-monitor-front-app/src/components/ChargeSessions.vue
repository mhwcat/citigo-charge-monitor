<template>
<article class="panel">
  <p class="panel-heading">Recent charge sessions</p>
  <div class="panel-block table-container">
  <div style="width: 100%">
      <table class="table is-fullwidth">
        <thead>
          <th class="has-text-left">Started at</th>
          <th class="has-text-left">Finished at</th>
          <th class="has-text-left">SOC</th>
        </thead>
        <tbody>
          <tr v-for="chargeSession in chargeSessions" :key="chargeSession.id">
            <td>{{ formatDt(chargeSession.startTime) }}</td>
            <td>{{ formatDt(chargeSession.stopTime) }}</td>
            <td>{{ chargeSession.startSoc }}% <span class="icon is-small"><font-awesome-icon :icon="['fas', 'arrow-right']" /></span> {{ chargeSession.stopSoc }}%</td>
          </tr>                                       
        </tbody>
      </table>
      <nav class="pagination is-justify-content-flex-end" role="navigation" aria-label="pagination">
        <span class="pagination-ellipsis"> {{ indexFrom }}-{{ indexTo }} / {{ totalCount }}</span>
        <button class="pagination-previous" :disabled="index <= 0" @click="handlePaginationPrevious">
          <span class="icon">
            <font-awesome-icon :icon="['fas', 'arrow-left']" />
          </span> 
        </button>
        <button class="pagination-next" :disabled="index >= maxIndex" @click="handlePaginationNext">          
          <span class="icon">
            <font-awesome-icon :icon="['fas', 'arrow-right']" />
          </span> 
        </button>
      </nav>  
    </div>
  </div>
</article>        
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ApiService from "@/services/ApiService";
import { ChargeSession } from "@/models/ChargeSession";
import { useToast } from "vue-toastification";
import { useStore } from "@/stores/store";
import { AxiosResponse } from "axios";
import { formatDt } from "@/main";

const PAGE_SIZE = 5;

export default defineComponent({
  name: "charge-sessions-list",
  setup() {
    const toast = useToast();
    const store = useStore();

    return { toast, store, formatDt }
  },  
  data() {
    return {
      index: 0,
      totalCount: 0,
      chargeSessions: [] as ChargeSession[],
    };
  },
  computed: {
    maxIndex(): number {
      return Math.max(this.totalCount - PAGE_SIZE, 0);
    },
    // These two are used for presentation, so they operate in 1..N range 
    // with exception of totalCount being zero (then we want to display "0-0 / 0")
    indexFrom(): number {
      if (this.totalCount == 0) {
        return 0;
      }

      return this.index + 1;
    },
    indexTo(): number {
      if (this.index + PAGE_SIZE > this.totalCount) {
        return this.totalCount;
      } else {
        return this.index + PAGE_SIZE;
      }
    }
  },
  methods: {
    fetchChargeSessions() {
      if (this.store.getters.vehicleId && this.store.getters.userSessionId) {
        ApiService.getChargeSessions(this.store.getters.vehicleId, this.index, PAGE_SIZE)
          .then((response: AxiosResponse) => {
            this.chargeSessions = response.data;
            this.totalCount = Number(response.headers["x-count"]);
          });
      }
    },
    handlePaginationPrevious() {
      if (this.index > 0) {
        this.index -= PAGE_SIZE;
        this.fetchChargeSessions();
      }
    },
    handlePaginationNext() {
      if (this.index < this.maxIndex) {
        this.index += PAGE_SIZE;
        this.fetchChargeSessions();
      }
    },
    resetPagination() {
      this.index = 0;
    }
  },
  created() {
    this.store.subscribe((mutation, state) => {
      if (mutation.type === "saveVehicle") {
          this.resetPagination();
          this.fetchChargeSessions();
      }
    });
  },
  mounted() {
    this.fetchChargeSessions();
  },
});
</script>