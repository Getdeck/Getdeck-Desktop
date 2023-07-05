<template>
  <v-container>
    <v-row>
      <h1>Create Cluster</h1>
    </v-row>
    <v-form v-model="formValid">
      <v-row>
        <v-col>
          <v-text-field
            v-model="clusterName"
            label="Cluster Name"
            outlined
            dense
            :rules="clusterNameRules"
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
            :rules="nodeCountRules"
            placeholder="1-3"
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
            hint="Please enter a valid port mapping, like 8080:8080 or 30812:9999."
          ></v-text-field>
        </v-col>
      </v-row>
    </v-form>
    <v-btn class="mb-1 mt-3" min-width="110" variant="flat" color="secondary" @click="addPortMapping" append-icon="mdi-plus-circle-outline">Ports</v-btn>
    <v-row>
      <v-col>
        <v-btn class="mb-1 mt-3" min-width="110" variant="flat" color="secondary" @click="createCluster" :disabled="!formValid">Create</v-btn>
      </v-col>
    </v-row>
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
import { ref, onMounted } from "vue";
import { ClustersService, ClusterRequest, ClusterParameter } from "beiboot-api-client";

import router from "../router";

const clusterName = ref("");
const nodeCount = ref(undefined);
const ports = ref(["6443:6443"]);

const snackbar = ref(false);
const snackbarInner = ref("Cluster creation failed");

const formValid = ref(false);

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
    console.log("creation success ", res);
    snackbar.value = true;
    snackbarInner.value = "Cluster created successfully.";
    router.push("/clusters");
  })
  .catch((err) => {
    console.error(err)
    snackbar.value = true;
    snackbarInner.value = "Cluster creation failed: Limits exceeded.";

  });
};

const clusterNameRules = [
  (value: string) => {
    if (value) return true;
    return "Cluster Name is required."
  },
  (value: string) => {
    if (!value.includes(" ")) return true;
    return "Cluster Name may not contain spaces."
  }
]

const nodeCountRules = [
  (value: number) => {
    if (0 < value  && value <= 3) return true;
    return "Node Count must be 1-3"
  }
]

const portRules = [
  (value: string) => {
    if (value.match("^[0-9]+:[0-9]+$")) return true;
    return "Please enter a valid port mapping, like 8080:8080 or 30812:9999."
  }
]

</script>

