/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Components
import App from './App.vue'

// Composables
import { createApp } from 'vue'

// Plugins
import { registerPlugins } from '@/plugins'

import { initKeycloak } from '@/auth/keycloak';
import router from '@/router'
import { Store } from "tauri-plugin-store-api";
import { OpenAPI } from "beiboot-api-client";
import { useAppStore } from './store/app';

const app = createApp(App)

registerPlugins(app)

OpenAPI.BASE = "https://api.beiboot.unikube.io"
const store = new Store(".settings.dat");

app.mount('#app')
store.get("token").then((token: any) => {
  if (token) {
    const keycloak = initKeycloak(token.value);
    const appStore = useAppStore();
    appStore.auth.keycloak = keycloak;
  } else {
    router.push("/login")
  }
})
