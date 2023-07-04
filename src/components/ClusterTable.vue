<template>
    <v-table>
        <thead>
            <tr>
                <th>Name</th>
                <th>State</th>
                <th>Sunset</th>
                <th>Lifetime/Timeout</th>
                <th>Actions</th>
            </tr>
        </thead>
        <tbody v-if="clusterList.length > 0">
            <tr v-for="cluster in clusterList" :key="cluster.name">
                <td>{{ cluster.name }}</td>
                <td>
                    <v-chip :color="getChipColor(cluster.state)" class="mt-1">
                        <v-icon start :icon="getIcon(cluster.state)"></v-icon>
                        {{ cluster.state }}
                    </v-chip>
                </td>
                <td>{{ getClusterSunset(cluster.sunset) }}</td>
                <td>{{ cluster.max_lifetime.value }} / {{ cluster.max_session_timeout.value }}</td>
                <td>
                    <v-tooltip text="Connect">
                    <template v-slot:activator="{ props }">
                        <v-btn v-bind="props" class="mt-2 mb-1 mr-2" size="small" variant="flat" color="secondary" icon="mdi-lan-connect" @click="clusterConnect(cluster.id, cluster.name)" v-if="store.connection.clusterName !== cluster.name" :disabled="!!store.connection.clusterName || cluster.state !== 'READY' || !store.docker"></v-btn>
                    </template>
                    </v-tooltip>
                    <v-tooltip text="Disconnect">
                    <template v-slot:activator="{ props }">
                        <v-btn v-bind="props" class="mt-2 mb-1 mr-2" size="small" variant="flat" color="secondary" icon="mdi-lan-disconnect" @click="clusterDisconnect(cluster.name)" v-if="store.connection.clusterName === cluster.name"></v-btn>
                    </template>
                    </v-tooltip>
                    <v-tooltip text="Delete">
                    <template v-slot:activator="{ props }">
                        <v-btn v-bind="props" class="mt-2 mb-1 mr-2" size="small" variant="flat" color="secondary" icon="mdi-delete" @click="clusterDelete(cluster.name, cluster.id)"></v-btn>
                    </template>
                    </v-tooltip>
                    <v-tooltip text="Shelf">
                    <template v-slot:activator="{ props }">
                        <v-btn v-bind="props" class="mt-2 mb-1 mr-2" size="small" variant="flat" color="secondary" icon="mdi-bookshelf" disabled></v-btn>
                    </template>
                    </v-tooltip>
                </td>
            </tr>
        </tbody>
        <tbody v-else>
          <tr>No clusters to list. Please create one first.</tr>
        </tbody>
    </v-table>
</template>

<script lang="ts" setup>
import { checkRunningConnects, connectCluster, disconnectCluster, writeKubeconfig } from "@/beibootctl";
import { useAppStore } from "@/store/app";
import { invoke } from "@tauri-apps/api/tauri";
import { ClusterStateResponse, ClustersService, ConnectionsService } from "beiboot-api-client";
import { ref, onMounted } from "vue";

const store = useAppStore();

const emit = defineEmits(["connected"]);

let clusterList = ref([] as ClusterStateResponse[]);

const getIcon = (state: string) => {
    switch (state) {
        case "READY":
            return "mdi-check";
        case "REQUESTED":
            return "mdi-clock";
        case "RUNNING":
            return "mdi-clock";
        case "PENDING":
            return "mdi-clock";
        default:
            return "mdi-alert";
    }
};

const getChipColor = (state: string) => {
    switch (state) {
        case "READY":
            return "success";
        case "REQUESTED":
            return "warning";
        case "RUNNING":
            return "warning";
        case "PENDING":
            return "warning";
        default:
            return "grey";
    }
};

const getClusterList = () => {
    ClustersService.clusterListClustersGet().then((res) => {
        clusterList.value = res.items;
    });
};

const clusterConnect = (clusterId: string, clusterName: string) => {
  ConnectionsService.ghostunnelConnectionsClusterIdGhostunnelGet(clusterId).then((res) => {
    const caCrt = res.mtls["ca.crt"];
    const clientCrt = res.mtls["client.crt"];
    const clientKey = res.mtls["client.key"];
    const ports = res.ports;

    connectCluster(clusterName, ports, caCrt, clientCrt, clientKey).then(() => {
      ClustersService.clusterKubeconfigClustersClusterIdKubeconfigGet(clusterId).then(async (res) => {
        const kubeconfigPath = await writeKubeconfig(clusterName, res);
        emit("connected", "Kubeconfig written to " + kubeconfigPath);
        invoke("establish_heartbeat_connection", {
          clusterId: clusterId,
          // @ts-ignore
          token: store.auth.token
        });
        store.connection.clusterName = clusterName;
        store.connection.clusterId = clusterId;
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
   disconnectCluster(clusterName).then(() => {
      emit("connected", "Connection closed successfully.");
      store.connection.clusterName = "";
      store.connection.kubeconfigPath = "";
      store.connection.connected = false;
      store.connection.clusterId = "";
    }).catch((err) => {
        console.log(err);
    });
};

const clusterDelete = (clusterName: string, clusterId: string) => {
    ClustersService.clusterDeleteClustersClusterIdDelete(clusterId).then((res) => {
        console.log("deletion success ", + res);
        clusterDisconnect(clusterName);
        setTimeout(() => {
            getClusterList();
        }, 2000);
    }).catch((err) => {
        console.log(err);
    });
};

onMounted(() => {
    getClusterList();
    setInterval(() => {
        getClusterList();
        checkRunningConnects();
        if (store.connection.clusterId) {
          invoke("establish_heartbeat_connection", {
            clusterId: store.connection.clusterId,
            // @ts-ignore
            token: store.auth.token
        }).catch(err => console.error(err));

        }
    }, 5000);
});

const getClusterSunset = (sunset: string) => {
  if (sunset) {
    return new Date(`${sunset}Z`).toLocaleTimeString()
  }
};
</script>
