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
import { OpenAPI } from "beiboot-api-client";
import { initKeycloak } from '@/auth/keycloak';
import * as Sentry from "@sentry/vue";
import { invoke } from '@tauri-apps/api/tauri';
import { useAppStore } from './store/app';

const app = createApp(App)

registerPlugins(app)

OpenAPI.BASE = "https://api.getdeck.dev"


if (process.env.NODE_ENV !== 'dev') {
  Sentry.init({
    app,
    dsn: "https://dd838e60a73d4902aa336ca38093e530@o146863.ingest.sentry.io/4505356797018112",
      integrations: [
      new Sentry.BrowserTracing({
        tracePropagationTargets: ["localhost", "tauri.localhost"],
        routingInstrumentation: Sentry.vueRouterInstrumentation(router),
      }),
      new Sentry.Replay(),
    ],
    // Performance Monitoring
    tracesSampleRate: 0.1,
    // Session Replay
    replaysSessionSampleRate: 0.1, // This sets the sample rate at 10%. You may want to change it to 100% while in development and then sample at a lower rate in production.
        replaysOnErrorSampleRate: 1.0, // If you're not already sampling the entire session, change the sample rate to 100% when sampling sessions where errors occur.
  });
}

app.mount('#app')

if (process.env.NODE_ENV !== 'debug') {
  document.addEventListener('contextmenu', event => event.preventDefault());
}

initKeycloak();
