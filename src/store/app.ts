// Utilities
import Keycloak from 'keycloak-js';
import { defineStore } from 'pinia'
import { Store } from 'tauri-plugin-store-api';

export const useAppStore = defineStore('appStore', {
  state: () => {
    return {
      connection: {
        clusterName: "",
        kubeconfigPath: "",
        connected: false,
      },
      auth: {
        authenticated: false,
        user: "",
        keycloak: null as Keycloak | null
      }
    }
  },
  actions: {
    logout() {
      const store = new Store(".settings.dat");
      store.set("token", "");
      store.clear();
      store.save();
      this.auth.authenticated = false;
      this.auth.user = "";
      if (this.auth.keycloak) {
        console.log(321);
        this.auth.keycloak.logout({})
      }
    }
  }
})
