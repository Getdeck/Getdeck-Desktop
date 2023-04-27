<template>
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
                    <v-btn class="mb-1 mt-3 mr-1" min-width="110" variant="outlined" @click="clusterConnect(cluster.name)" v-if="store.connection.clusterName !== cluster.name" :disabled="!!store.connection.clusterName || cluster.state !== 'READY'">Connect</v-btn>
                    <v-btn class="mb-1 mt-3 mr-1" min-width="110" variant="outlined" @click="clusterDisconnect(cluster.name)" v-if="store.connection.clusterName === cluster.name">Disconnect</v-btn>
                    <v-btn class="mb-1 mt-3" min-width="110" variant="outlined">Edit</v-btn> <br>
                    <v-btn class="mb-3 mr-1" min-width="110" variant="outlined" @click="clusterDelete(cluster.name)">Delete</v-btn>
                    <v-btn class="mb-3" min-width="110" variant="outlined">Shelf</v-btn>
                </td>
            </tr>
        </tbody>
    </v-table>
</template>

<script lang="ts" setup>
import { connectCluster, disconnectCluster, writeKubeconfig } from "@/beibootctl";
import { useAppStore } from "@/store/app";
import { ClusterStateResponse, ClustersService, ConnectionsService } from "beiboot-api-client";
import { ref, onMounted } from "vue";

const store = useAppStore();

const emit = defineEmits(["connected"]);

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
      ClustersService.clusterKubeconfigClustersClusterNameKubeconfigGet(clusterName).then(async (res) => {
        const kubeconfigPath = await writeKubeconfig(clusterName, res);
        emit("connected", "Kubeconfig written to " + kubeconfigPath);
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
      emit("connected", "Connection closed successfully.");
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
