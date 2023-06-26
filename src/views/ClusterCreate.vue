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
          :spellcheck="false"
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
      <v-col v-for="(_,index) in ports" :key="index">
        <v-text-field
          v-model="ports[index]"
          label="Ports"
          outlined
          dense
          required
          clearable
          @click:clear="handlePortClear(index)"
        ></v-text-field>
      </v-col>
    </v-row>
    <v-btn class="mb-1 mt-3" min-width="110" variant="flat" color="secondary" @click="addPortMapping" append-icon="mdi-plus-circle-outline">Ports</v-btn>
    <v-row>
      <v-col>
        <v-btn class="mb-1 mt-3" min-width="110" variant="flat" color="secondary" @click="createCluster">Create</v-btn>
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
const ports = ref(["6443:6443"]);

const addPortMapping = () => {
  ports.value.push("host");
};

const handlePortClear = (index: number) => {
    ports.value.splice(index, 1);
};


const createCluster = () => {
  const clusterReq: ClusterRequest = {
    name: clusterName.value,
    parameters: [
      {
        name: ClusterParameter.NODE_COUNT,
        value: nodeCount.value,
      },
      {
        name: ClusterParameter.PORTS,
        value: ports.value,
      },
    ],
  };
  ClustersService.clusterCreateClustersPost(clusterReq).then((res) => {
    console.log("creation success ", +res);
    router.push("/clusters");
  })
  .catch((err) => {
    console.log("creation failed ", +err);
  });
};

</script>

