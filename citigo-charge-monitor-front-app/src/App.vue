<template>
  <router-view/>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useRouter } from "vue-router";
import { useStore } from "@/stores/store";
import { useToast } from "vue-toastification";
import apiClient from "@/http-common";

export default defineComponent({
  name: "main-app",
  setup() {
    const toast = useToast();
    const store = useStore();
    const router = useRouter();

    return { toast, store, router }
  },
  created() {
    this.setupApiClientInterceptors();

    this.store.subscribe((mutation, state) => {
      if (mutation.type === "initialiseStore" || mutation.type === "saveUserSession") {
        if (state.userSession) {
          this.router.push("/");     
        } else {
          this.router.push("/login");
        }
      }
    }); 

    this.store.commit("initialiseStore");
  },
  methods: {
    setupApiClientInterceptors() {
      apiClient.interceptors.request.use((config) => {
        const token = this.store.getters.userSessionId;

        if (token) {
          if (!config.headers) {
            config.headers = {};
          }
            
          config.headers.Authorization =  "Bearer " + token;
        }
      
        return config;
      }, error => error); 

      apiClient.interceptors.response.use((response) => response, (error) => {
        if (error.response.status == 401) {
          this.store.commit("saveUserSession", null);
          this.toast.error("Authorization error", { timeout: 2000 });
        } else {
          this.toast.error("Failed: " + error, { timeout: 2000 });
        }

        throw error;
      }); 
    }
  }
});
</script>

<style>
.panel-heading {
  line-height: 0.75 !important;
}
</style>