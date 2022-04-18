import { createApp } from "vue";
import App from "./App.vue";
import "bulma";
import "bulma-slider";
import Toast from "vue-toastification";
import "vue-toastification/dist/index.css";
import { store, key } from "@/stores/store";
import { library } from "@fortawesome/fontawesome-svg-core";
import { faLock, faCar, faRightFromBracket, faUser, faArrowLeft, faArrowRight, faPlugCircleXmark, faPlugCircleBolt } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { createRouter, createWebHistory } from "vue-router";
import LoginView from "@/views/LoginView.vue";
import MainView from "@/views/MainView.vue";
import dayjs from "dayjs";

library.add(faUser, faLock, faCar, faRightFromBracket, faArrowLeft, faArrowRight, faPlugCircleXmark, faPlugCircleBolt);

const routes = [
  { path: "/login", component: LoginView },
  { path: "/", component: MainView },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

const app = createApp(App);

export function formatDt(datetime: string | null) {
  if (datetime) {
    return dayjs(datetime).format("YYYY-MM-DD HH:mm:ss");
  } else {
    return null;
  }
}

app
  .use(router)
  .use(store, key)
  .use(Toast, { maxToasts: 1 })
  .component("font-awesome-icon", FontAwesomeIcon)
  .mount("#app");