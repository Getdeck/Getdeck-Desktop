<template>
  <v-app>
    <v-app-bar
      elevation="0"
      color="primary"
      :border="true"
    >
        <router-link to="/home">
            <v-img src="@/../app-icon.png" cover></v-img>
            <h2>Getdeck Beiboot</h2>
        </router-link>
        <v-spacer></v-spacer>
        <v-btn variant="outlined">GitHub</v-btn>
        <router-link to="/login">
            <v-btn class="ml-3" variant="outlined">Login</v-btn>
        </router-link>
    </v-app-bar>
    <v-card>
      <v-navigation-drawer
          location="left"
          v-mode="drawer"
          order="2"
          permanent
        >
            <v-list
          >
          <v-list-item v-for="item in items" :key="item.title">
          <router-link :to="item.value">{{ item.title }}</router-link>
          </v-list-item>
          </v-list>
          <template v-slot:append>
              <v-list>
                <v-list-item title="Settings" value="settings" />
              </v-list>
          </template>
        </v-navigation-drawer>
      </v-card>

    <default-view />
    <v-bottom-navigation order="1" bg-color="secondary" elevation="0" :border="true">
       <v-btn color="getdeckPrimary" v-if="user" variant="tonal">Hi, {{ user.firstName }}</v-btn>
       <v-btn color="getdeckPrimary" v-else variant="tonal">Not logged in</v-btn>
       <v-spacer></v-spacer>
       <v-btn color="primary" variant="tonal">Engine: Docker</v-btn>
       <v-btn color="primayr" v-if="appStore.connection.connected" variant="tonal">Connected to {{ appStore.connection.clusterName }}</v-btn>
       <v-btn color="primayr" v-else variant="tonal">Not connected</v-btn>
  </v-bottom-navigation>
  </v-app>
</template>

<script lang="ts" setup>
  import { ref, onMounted } from 'vue';
  import { Store } from "tauri-plugin-store-api";
  import DefaultView from './View.vue'
  import { useAppStore } from '@/store/app';

  const store = new Store(".settings.dat");
  const appStore = useAppStore();

  store.get("user").then((res) => console.log(res.value))

  const user = ref(null);

  onMounted(async () => {
    const storedUser = await store.get("user");
    user.value = storedUser.value;
  });

  const drawer = ref(true);
  const items = ref([
    { title: 'Clusters', icon: 'mdi-home', value: '/clusters' },
    { title: 'Inventory', icon: 'mdi-account', value: '/' },
    { title: 'Local Containers', icon: 'mdi-email', value: '/' },
  ])

</script>
