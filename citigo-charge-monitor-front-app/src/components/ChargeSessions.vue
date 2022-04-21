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
        <span class="pagination-ellipsis">{{ index + 1 }}-{{ (index + pageSize) > (maxPage * pageSize) ? (maxPage * pageSize) + 1 : (index + pageSize) }} / {{ (maxPage * pageSize) + 1 }}</span>
        <button class="pagination-previous" :disabled="currentPage <= 0" @click="handlePaginationPrevious">
          <span class="icon">
            <font-awesome-icon :icon="['fas', 'arrow-left']" />
          </span> 
        </button>
        <button class="pagination-next" :disabled="currentPage >= maxPage" @click="handlePaginationNext">          
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

export default defineComponent({
  name: "charge-sessions-list",
  setup() {
    const toast = useToast();
    const store = useStore();

    return { toast, store, formatDt }
  },  
  data() {
    return {
      currentPage: 0,
      pageSize: 5,
      maxPage: 0,
      chargeSessions: [] as ChargeSession[],
    };
  },
  computed: {
    index(): number {
      return this.currentPage * this.pageSize;
    }
  },
  methods: {
    fetchChargeSessions() {
      if (this.store.getters.vehicleId && this.store.getters.userSessionId) {
        ApiService.getChargeSessions(this.store.getters.vehicleId, this.index, this.pageSize)
          .then((response: AxiosResponse) => {
            this.chargeSessions = response.data;
            this.maxPage = Math.floor(Number(response.headers["x-count"]) / this.pageSize);
          });
      }
    },
    handlePaginationPrevious() {
      if (this.currentPage > 0) {
        this.currentPage -= 1;
        this.fetchChargeSessions();
      }
    },
    handlePaginationNext() {
      if (this.currentPage < this.maxPage) {
        this.currentPage += 1;
        this.fetchChargeSessions();
      }
    }
  },
  created() {
    this.store.subscribe((mutation, state) => {
      if (mutation.type === "saveVehicle") {
          this.fetchChargeSessions();
      }
    });
  },
  mounted() {
    this.fetchChargeSessions();
  },
});
</script>