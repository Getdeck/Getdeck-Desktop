<template>
  <v-app>
    <v-app-bar
      elevation="0"
      :border="true"
    ></v-app-bar>
    <v-card>
      <v-navigation-drawer
          location="left"
          v-mode="drawer"
          order="2"
          permanent
        >
            <v-list
            :items="items"
          ></v-list>
          <template v-slot:append>
              <v-list>
                <v-list-item title="Settings" value="settings" />
              </v-list>
          </template>
        </v-navigation-drawer>
      </v-card>

    <default-view />
    <v-bottom-navigation order="1" elevation="0" :border="true">
        
  </v-bottom-navigation>
  </v-app>
</template>

<script lang="ts" setup>
  import { ref } from 'vue';
  import DefaultView from './View.vue'

  const drawer = ref(true);
  const items = ref([
    { title: 'Clusters', icon: 'mdi-home', value: '/' },
    { title: 'Inventory', icon: 'mdi-account', value: '/' },
    { title: 'Local Containers', icon: 'mdi-email', value: '/' },
  ])

</script>
