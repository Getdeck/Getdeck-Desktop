<template>
  <v-app>
    <v-card>
      <v-navigation-drawer
          location="left"
          v-mode="drawer"
          order="2"
          permanent
          color="#f3f3f3"
          width="250"
        >
        <v-list>
          <v-list-item>
            <v-img src="/logo.svg" width="200" class="mb-5"></v-img>
          </v-list-item>
          <v-list-item v-for="item in items" :key="item.title" class="ml-1" :to="item.value" :active="false">
            <template v-slot:prepend>
              <v-icon :icon="item.icon" :color="route.path === item.value ? '#ff165d' : 'secondary-2'"></v-icon>
            </template>
            <v-list-item-title :class="{'font-weight-bold': route.path === item.value}">{{ item.title }}</v-list-item-title>
          </v-list-item>
          </v-list>
          <template v-slot:append>
              <v-list>
                <v-list-item title="Settings" value="settings" :active="route.path === '/settings'">
                <template v-slot:prepend>
                  <v-icon icon="mdi-cog" :color="route.path === '/settings' ? '#ff165d' : 'iconColor'"></v-icon>
                </template>
                </v-list-item>
              </v-list>
          </template>
        </v-navigation-drawer>
      </v-card>

    <default-view />
    <v-bottom-navigation order="1" bg-color="secondary" elevation="0" :border="true">
      <v-menu
      v-if="user"
      open-on-hover
    >
      <template v-slot:activator="{ props }">
        <v-btn
          color="getdeckPrimary"
          v-bind="props"
        >
        Hi, {{ user.firstName }}
        </v-btn>
      </template>

      <v-list>
        <v-list-item
          v-for="(item, index) in accountItems"
          :key="index"
          :to="item.value"
        >
          <v-list-item-title>{{ item.title }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
       <v-btn v-else>Not logged in</v-btn>
       <v-spacer></v-spacer>
       <v-btn>Engine: Docker</v-btn>
       <v-btn v-if="appStore.connection.connected">Connected to {{ appStore.connection.clusterName }}</v-btn>
       <v-btn v-else>Not connected</v-btn>
  </v-bottom-navigation>
  </v-app>
</template>

<script lang="ts" setup>
  import { ref, onMounted } from 'vue';
  import { Store } from "tauri-plugin-store-api";
  import DefaultView from './View.vue'
  import { useAppStore } from '@/store/app';
import { useRoute } from 'vue-router';

  const store = new Store(".settings.dat");
  const route = useRoute();
  const appStore = useAppStore();

  store.get("user").then((res) => console.log(res.value))

  const user = ref(null);

  onMounted(async () => {
    const storedUser = await store.get("user");
    user.value = storedUser.value;
  });

  const drawer = ref(true);
  const items = ref([
    { title: 'Clusters', icon: 'mdi-server', value: '/clusters' },
    { title: 'Inventory', icon: 'mdi-bookshelf', value: '/' },
    { title: 'Local Containers', icon: 'mdi-package', value: '/' },
  ])

  const accountItems = ref([
    { title: 'Logout', value: '/logout' },
  ])

</script>
