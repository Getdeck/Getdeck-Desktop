<template>
  <v-container>
    <v-img src="/logo.svg" width="200" class="mb-5"></v-img>
    <h2>Login</h2>
    <v-form class="mt-6">
      <v-row>
        <v-col cols="5">
          <v-text-field v-model="username" label="Username" outlined dense :spellcheck="false" required></v-text-field>
        </v-col>
        <v-col cols="5">
          <v-text-field v-model="password" label="Password" outlined :spellcheck="false" dense required
            type="password"></v-text-field>
        </v-col>
      </v-row>
      <v-btn @click="login" variant="flat" color="secondary">Login</v-btn>
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

