/**
 * plugins/index.ts
 *
 * Automatically included in `./src/main.ts`
 */
// Plugins
import { loadFonts } from './webfontloader';
import vuetify from './vuetify';
import pinia from '../store';
import router from '../router';
export function registerPlugins(app) {
    loadFonts();
    app
        .use(vuetify)
        .use(router)
        .use(pinia);
}
