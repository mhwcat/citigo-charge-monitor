<template>
<section class="is-primary is-fullheight">
  <div class="hero-body">
    <div class="container">
      <div class="columns is-centered">
        <div class="column is-5-tablet is-4-desktop is-3-widescreen">
          <h1 class="title is-4 is-centered">Citigo Charge Monitor</h1>
          <form action="" class="box" @submit.prevent="handleSubmit">
            <div class="field">
              <label for="" class="label">Username</label>
              <div class="control has-icons-left">
                <input type="username" placeholder="Username" class="input" v-model="username" required>
                <span class="icon is-small is-left">
                  <font-awesome-icon :icon="['fas', 'user']" />
                </span>
              </div>
            </div>
            <div class="field">
              <label for="" class="label">Password</label>
              <div class="control has-icons-left">
                <input type="password" placeholder="*******" class="input" v-model="password" required>
                <span class="icon is-small is-left">
                  <font-awesome-icon :icon="['fas', 'lock']" />
                </span>                
              </div>
            </div>
            <div class="field">
              <button class="button is-success" :disabled="loggingIn">
                Login
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</section>       
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ApiService from "@/services/ApiService";
import { useToast } from "vue-toastification";
import { useStore } from "@/stores/store";
import { useRouter } from "vue-router";

export default defineComponent({
  name: "login-component",
  setup() {
    const toast = useToast();
    const store = useStore();
    const router = useRouter();

    return { toast, store, router }
  },  
  data() {
    return {
      username: null,
      password: null,
      loggingIn: false,
    };
  },
  methods: {
    handleSubmit() {
      if (this.username && this.password) {
        this.loggingIn = true;
            
        ApiService.login(this.username, this.password)
          .then((response: any) => {
            this.store.commit("saveUserSession", response.data);
            this.toast.success("Logged in as " + this.username, {
              timeout: 2000
            });
          })
          .finally(() => {
            this.loggingIn = false;
          });
      }
    },
  }
});
</script>