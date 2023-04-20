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

const app = createApp(App)

registerPlugins(app)

OpenAPI.BASE = "https://api.beiboot.unikube.io"
const store = new Store(".settings.dat");
store.get("token").then((token: any) => {
    if (token) {
      initKeycloak(token.value)
    } else {
      router.push("/login")
    }
})

app.mount('#app')
