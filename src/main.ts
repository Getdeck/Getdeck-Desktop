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
import { Store } from "tauri-plugin-store-api";
import { OpenAPI } from "beiboot-api-client";

const app = createApp(App)

registerPlugins(app)

const store = new Store(".settings.dat");
store.get("token").then((token: any) => {
    console.error("initing keycloak")
    console.log(token)
    initKeycloak(token.value)
    OpenAPI.BASE = "https://api.beiboot.unikube.io"
    OpenAPI.HEADERS = {
        'Authorization': `Bearer ${token.value.token}`
    };
})

app.mount('#app')
