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

import router from '@/router'
import { Store } from "tauri-plugin-store-api";
import { OpenAPI } from "beiboot-api-client";
import { useAppStore } from './store/app';
import { initKeycloak } from '@/auth/keycloak';

import Keycloak from 'keycloak-js';

const app = createApp(App)

registerPlugins(app)

OpenAPI.BASE = "https://api.getdeck.dev"

app.mount('#app')
initKeycloak();
