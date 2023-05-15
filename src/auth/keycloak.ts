import Keycloak from 'keycloak-js';
import axios from 'axios';
import { Store } from "tauri-plugin-store-api";
import { OpenAPI } from "beiboot-api-client";

import router from "@/router";
import { useAppStore } from "@/store/app";

export interface Token {
    token: string;
    refreshToken: string;
}

export async function getInitialToken(user: string, password: string): Promise<Token> {
    const store = new Store(".settings.dat");
    const params = new URLSearchParams();
    params.append('username', user);
    params.append('password', password);
    params.append('grant_type', 'password');
    params.append('client_id', 'beiboot-api');

    const res = await axios.post('https://login.beiboot.unikube.io/auth/realms/getdeck-beiboot/protocol/openid-connect/token', params);
    const token = <Token>{token: res.data.access_token, refreshToken: res.data.refresh_token};
    store.set("token", { value: token  });
    OpenAPI.TOKEN = token.token;
    initKeycloak(token);
    router.push("/clusters");
    return token;
}

let initOptions = {
    url: 'https://login.beiboot.unikube.io/auth/',
    realm: 'getdeck-beiboot',
    clientId: 'beiboot-api'
}

export function initKeycloak(token: Token): Keycloak {
    const keycloak = new Keycloak(initOptions);
    const appStore = useAppStore();
    appStore.auth.keycloak = keycloak;
    keycloak.init({
        token: token.token,
        refreshToken: token.refreshToken,
        checkLoginIframe: false,
    }).then((authenticated) => {
        if (authenticated) {
            console.log("authenticated");
            OpenAPI.TOKEN = keycloak.token;
            keycloak.loadUserProfile().then((profile) => {
                const store = new Store(".settings.dat");
                appStore.auth.authenticated = true;
                appStore.auth.user = profile.firstName || "";
                store.set("user", { value: profile });
                router.push("/clusters");
            });
        } else {
            console.log("not authenticated");
        }
    }).catch((err) => {
        console.log(err);
        router.push("/login");
    });

    setInterval(() => {
        keycloak.updateToken(0).then((refreshed) => {
            console.debug(refreshed)
            if (refreshed) {
                console.debug('Token refreshed' + refreshed);
                OpenAPI.TOKEN = keycloak.token;
            } else {
                console.debug('Token not refreshed, still valid.');
            }
        }).catch(() => {
            console.debug('Failed to refresh token');
        });
    }, 6000)
    return keycloak
}
