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
        <v-btn class="ml-3" variant="outlined">Login</v-btn>
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
       <v-btn color="getdeckPrimary"  variant="tonal">Not logged in</v-btn> 
       <v-spacer></v-spacer>
       <v-btn color="primary" variant="tonal">Engine: Docker</v-btn>
       <v-btn color="primayr" variant="tonal">Not Connected</v-btn>
  </v-bottom-navigation>
  </v-app>
</template>

<script lang="ts" setup>
  import { ref } from 'vue';
  import DefaultView from './View.vue'

  const drawer = ref(true);
  const items = ref([
    { title: 'Clusters', icon: 'mdi-home', value: '/clusters' },
    { title: 'Inventory', icon: 'mdi-account', value: '/' },
    { title: 'Local Containers', icon: 'mdi-email', value: '/' },
  ])

</script>
