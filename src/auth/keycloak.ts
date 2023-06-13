import Keycloak from 'keycloak-js';
import axios from 'axios';
import { Store } from "tauri-plugin-store-api";
import { OpenAPI } from "beiboot-api-client";

import router from "@/router";
import { useAppStore } from "@/store/app";
import { open } from "@tauri-apps/api/shell";
import { listen } from "@tauri-apps/api/event";
import { startOAuthServer } from '@/beibootctl';

export interface Token {
    token: string;
    refreshToken: string;
}

let initOptions = {
  url: 'https://login.beiboot.unikube.io/auth/',
    realm: 'getdeck-beiboot',
  clientId: 'beiboot-api'
}

const openLoginBrowser = async (keycloak: Keycloak.KeycloakInstance) => {
  const port = await startOAuthServer();
  const redirectUri = `http://localhost:${port}`;
  keycloak.init({
    checkLoginIframe: false,
  });
  keycloak.login({
    redirectUri: redirectUri
  });
  console.log("opening...")

}

export async function getInitialToken(keycloak: Keycloak.KeycloakInstance) {
    const store = new Store(".settings.dat");
    listen('redirect_uri', (data) => {
      console.log(data.payload);
    });
    openLoginBrowser(keycloak);
}

export async function initKeycloak() {
  let initOptions = {
    url: 'https://login.beiboot.unikube.io/auth/',
      realm: 'getdeck-beiboot',
    clientId: 'beiboot-api'
  }
  const appStore = useAppStore();
  const store = new Store(".settings.dat");

  const storeToken = await store.get("token");
  const storeRefreshToken = await store.get("refreshToken");

  const keycloak = new Keycloak(initOptions);
  keycloak.init({
    checkLoginIframe: false,
    onLoad: 'check-sso',
    enableLogging: true,
    // @ts-ignore
    token: storeToken?.value,
    // @ts-ignore
    refreshToken: storeRefreshToken?.value
  }).then(async (authenticated) => {
    console.log(authenticated)
    if (authenticated) {
      console.log(keycloak.token)
      store.set("token", {value: keycloak.token})
      store.set("refreshToken", {value: keycloak.refreshToken})
      OpenAPI.TOKEN = keycloak.token;
      await store.save();
      appStore.auth.authenticated = true;
      appStore.auth.keycloak = keycloak;
      router.push("/clusters");
    } else {
      getInitialToken(keycloak);
    }
  }).catch((err) => {
      console.log(err)
    })
  console.log(keycloak)

}
