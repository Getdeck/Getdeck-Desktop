<template>
  <v-container>
    <v-row>
      <v-col>
        <h1>Clusters</h1>
      </v-col>
      <v-col class="text-end">
        <router-link to="clusters/create" class="router-link-btn">
          <v-btn append-icon="mdi-plus-circle-outline" min-width="110" variant="flat" color="secondary">
            Create Cluster
          </v-btn>
        </router-link>
      </v-col>
    </v-row>
    <cluster-table @connected="handleConnectionMessage"/>
  </v-container>
  <v-snackbar
      v-model="snackbar"
    >
      {{ snackbarInner }}

      <template v-slot:actions>
        <v-btn
          color="pink"
          variant="text"
          @click="snackbar = false"
        >
          Close
        </v-btn>
      </template>
    </v-snackbar>
</template>

<script lang="ts" setup>
import ClusterTable from "@/components/ClusterTable.vue";
import { ref } from "vue";

const snackbar = ref(false);
const snackbarInner = ref("");

function handleConnectionMessage(message: string) {
  snackbarInner.value = message;
  snackbar.value = true;
}
</script>
