import { InjectionKey } from "vue";
import { createStore, useStore as baseUseStore, Store } from "vuex";
import UserSession from "@/models/UserSession";
import Vehicle from "@/models/Vehicle";

export interface State {
  userSession: UserSession | null,
  vehicle: Vehicle | null
}

export const key: InjectionKey<Store<State>> = Symbol()

export const store = createStore<State>({
  state: {
    userSession: null,
    vehicle: null,
  },
  mutations: {
    initialiseStore(state) {
      const savedStore = localStorage.getItem("store");

      if (savedStore) {
        this.replaceState(
          Object.assign(this.state, JSON.parse(savedStore))
        );
      }
    }, 
    saveUserSession(state, userSession) {
      state.userSession = userSession;
    },
    saveVehicle(state, vehicle) {
      state.vehicle = vehicle;
    }    
  },
  getters: {
    userSessionId: (state) => {
      return state.userSession?.id;
    },
    vehicleId: (state) => {
      return state.vehicle?.id;
    },
    vehicleVin: (state) => {
      return state.vehicle?.vin;
    }    
  }
})

store.subscribe((mutation, state) => {
  localStorage.setItem("store", JSON.stringify(state));
});

// define your own `useStore` composition function
export function useStore() {
  return baseUseStore(key)
}
