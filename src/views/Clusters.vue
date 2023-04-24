<template>
  <v-container class="fill-height">
    <v-row>
      <v-col>
        <h1>Clusters</h1>
      </v-col>
      <v-spacer></v-spacer>
      <v-col>
        <router-link to="clusters/create">
          <v-btn class="" min-width="110" variant="outlined">Create</v-btn>
        </router-link>
      </v-col>
    </v-row>
    <v-table>
        <thead>
            <tr>
                <th>Name</th>
                <th>State</th>
                <th>Actions</th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="cluster in clusterList" :key="cluster.name">
                <td>{{ cluster.name }}</td>
                <td>{{ cluster.state }}</td>
                <td>
                    <v-btn class="mb-1 mt-3 mr-1" min-width="110" variant="outlined" @click="clusterConnect(cluster.name)">Connect</v-btn>
                    <v-btn class="mb-1 mt-3 mr-1" min-width="110" variant="outlined" @click="clusterDisconnect(cluster.name)">Disconnect</v-btn>
                    <v-btn class="mb-1 mt-3" min-width="110" variant="outlined">Edit</v-btn> <br>
                    <v-btn class="mb-3 mr-1" min-width="110" variant="outlined" @click="clusterDelete(cluster.name)">Delete</v-btn>
                    <v-btn class="mb-3" min-width="110" variant="outlined">Shelf</v-btn>
                </td>
            </tr>
        </tbody>
    </v-table>
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
import { connectCluster, disconnectCluster, writeKubeconfig } from "@/beibootctl";
import { useAppStore } from "@/store/app";
import { ClusterStateResponse, ClustersService, ConnectionsService } from "beiboot-api-client";
import { ref, onMounted } from "vue";

const store = useAppStore();

let snackbar = ref(false);
let snackbarInner = ref("");

let clusterList = ref([] as ClusterStateResponse[]);

const getClusterList = () => {
    ClustersService.clusterListClustersGet().then((res) => {
        clusterList.value = res.items;
    });
};

const clusterConnect = (clusterName: string) => {
  ConnectionsService.ghostunnelConnectionsClusterNameGhostunnelGet(clusterName).then((res) => {
    const caCrt = res.mtls["ca.crt"];
    const clientCrt = res.mtls["client.crt"];
    const clientKey = res.mtls["client.key"];
    const ports = res.ports;

    connectCluster(clusterName, ports, caCrt, clientCrt, clientKey).then((res) => {
      snackbarInner.value = "Connection established successfully.";
      snackbar.value = true;
      ClustersService.clusterKubeconfigClustersClusterNameKubeconfigGet(clusterName).then(async (res) => {
        const kubeconfigPath = await writeKubeconfig(clusterName, res);
        snackbarInner.value = "Kubeconfig written to " + kubeconfigPath;
        snackbar.value = true;
        store.connection.clusterName = clusterName;
        store.connection.kubeconfigPath = kubeconfigPath;
        store.connection.connected = true;

      }).catch((err) => {
        console.error(err);
      });
    }).catch((err) => {
      console.error(err);
    });

  }).catch((err) => {
    console.error(err);
  });


};

const clusterDisconnect = (clusterName: string) => {
   disconnectCluster(clusterName).then((res) => {
      snackbarInner.value = "Connection closed successfully.";
      snackbar.value = true;
      store.connection.clusterName = "";
      store.connection.kubeconfigPath = "";
      store.connection.connected = false;
    }).catch((err) => {
        console.log(err);
    });
};

const clusterDelete = (clusterName: string) => {
    ClustersService.clusterDeleteClustersClusterNameDelete(clusterName).then((res) => {
        console.log("deletion success ", + res);
        setTimeout(() => {
            getClusterList();
        }, 2000);
    }).catch((err) => {
        console.log(err);
    });
};

onMounted(() => {
    getClusterList();
});

</script>
