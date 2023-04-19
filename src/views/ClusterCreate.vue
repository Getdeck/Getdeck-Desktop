<template>
  <v-container>
    <v-row>
      <h1>Create Cluster</h1>
    </v-row>
    <v-row>
      <v-col>
        <v-text-field
          v-model="clusterName"
          label="Cluster Name"
          outlined
          dense
          required
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          v-model="nodeCount"
          label="Node Count"
          outlined
          dense
          required
        ></v-text-field>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-btn class="mb-1 mt-3" min-width="110" variant="outlined" @click="createCluster">Create</v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { ref, onMounted } from "vue";
import { ClustersService, ClusterRequest, ClusterParameter } from "beiboot-api-client";

import router from "../router";

const clusterName = ref("");
const nodeCount = ref(1);

const createCluster = () => {
  const clusterReq: ClusterRequest = {
    name: clusterName.value,
    parameters: [
      {
        name: ClusterParameter.NODE_COUNT,
        value: nodeCount.value,
      },
    ],
  };
  ClustersService.clusterCreateClustersPost(clusterReq).then((res) => {
    console.log("creation success ", +res);
    router.push("/clusters");
  });
};

</script>

