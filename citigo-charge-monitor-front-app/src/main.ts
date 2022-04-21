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
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// Discard more than one toast of the same type for readability
const filterBeforeCreate = (toast: { type: any; }, toasts: { filter: (arg0: (t: any) => boolean) => { (): any; new(): any; length: number; }; }) => {
  if (toasts.filter((t: { type: any; }) => t.type === toast.type).length !== 0) {
    return false;
  }

  return toast;
};

export function formatDt(datetime: string | null) {
  if (datetime) {
    return dayjs(datetime).format("YYYY-MM-DD HH:mm:ss");
  } else {
    return null;
  }
}

const app = createApp(App);

app
  .use(router)
  .use(store, key)
  .use(Toast, { maxToasts: 1, filterBeforeCreate })
  .component("font-awesome-icon", FontAwesomeIcon)
  .mount("#app");