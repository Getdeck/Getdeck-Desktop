/**
 * plugins/vuetify.ts
 *
 * Framework documentation: https://vuetifyjs.com`
 */
/// <reference types="vuetify" />
import '@mdi/font/css/materialdesignicons.css';
import 'vuetify/styles';
declare const _default: {
    install: (app: import("vue").App<any>) => void;
    defaults: import("vue").Ref<import("vuetify").DefaultsInstance>;
    display: import("vuetify").DisplayInstance;
    theme: import("vuetify").ThemeInstance & {
        install: (app: import("vue").App<any>) => void;
    };
    icons: Record<string, any>;
    locale: {
        isRtl: import("vue").Ref<boolean>;
        rtl: import("vue").Ref<Record<string, boolean>>;
        rtlClasses: import("vue").Ref<string>;
        name: string;
        messages: import("vue").Ref<import("vuetify").LocaleMessages>;
        current: import("vue").Ref<string>;
        fallback: import("vue").Ref<string>;
        t: (key: string, ...params: unknown[]) => string;
        n: (value: number) => string;
        provide: (props: import("vuetify").LocaleOptions) => import("vuetify").LocaleInstance;
    };
};
export default _default;
