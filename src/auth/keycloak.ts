import Keycloak from 'keycloak-js';
import axios from 'axios';
import { Store } from "tauri-plugin-store-api";

import router from "@/router";

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
    console.log("written token to store")
    router.push("home");
    initKeycloak(token);
    return token;
}

let initOptions = {
    url: 'https://login.beiboot.unikube.io/auth/',
    realm: 'getdeck-beiboot',
    clientId: 'beiboot-api'
}

export function initKeycloak(token: Token) {
    const keycloak = new Keycloak(initOptions);
    keycloak.init({
        token: token.token,
        refreshToken: token.refreshToken,
        checkLoginIframe: false,
    }).then((authenticated) => {
        if (authenticated) {
            console.log("authenticated");
            keycloak.loadUserProfile().then((profile) => {
                const store = new Store(".settings.dat");
                store.set("user", { value: profile });
            });
        } else {
            console.log("not authenticated");
        }
    }).catch((err) => {
        console.log(err);
        router.push("/login");
    });

    setInterval(() => {
        keycloak.updateToken(70).then((refreshed) => {
            if (refreshed) {
                console.log('Token refreshed' + refreshed);
            } else {
                console.log('Token not refreshed, still valid.');
            }
        }).catch(() => {
            console.log('Failed to refresh token');
        });
    }, 60000)
}
