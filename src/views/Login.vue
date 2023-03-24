<template>
  <v-container class="fill-height">
    <h1>Login</h1>
    <v-form>
        <v-text-field
            v-model="username"
            label="Username"
            outlined
            dense
            required
        ></v-text-field>
        <v-text-field
            v-model="password"
            label="Password"
            outlined
            dense
            required
        ></v-text-field>
        <v-btn @click="login">Login</v-btn>
    </v-form>
  </v-container>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { Store } from "tauri-plugin-store-api";
import { getInitialToken } from "@/auth/keycloak";

const username = ref("");
const password = ref("");

const login = () => {
    getInitialToken(username.value, password.value);
}


const store = new Store(".settings.dat");
/* let clusterList = ClustersService.clusterListClustersGet(1) */
</script>

