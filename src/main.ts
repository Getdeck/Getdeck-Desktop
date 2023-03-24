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

import { initKeycloak, getInitialToken } from '@/auth/keycloak';
import { Store } from "tauri-plugin-store-api";

const app = createApp(App)

registerPlugins(app)

const store = new Store(".settings.dat");
store.get("token").then((token) => {
    initKeycloak(token.value)
})

app.mount('#app')
