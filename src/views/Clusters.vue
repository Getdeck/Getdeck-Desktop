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
                    <v-btn class="mb-1 mt-3 mr-1" min-width="110" variant="outlined">Connect</v-btn>
                    <v-btn class="mb-1 mt-3" min-width="110" variant="outlined">Edit</v-btn> <br>
                    <v-btn class="mb-3 mr-1" min-width="110" variant="outlined" @click="clusterDelete(cluster.name)">Delete</v-btn>
                    <v-btn class="mb-3" min-width="110" variant="outlined">Shelf</v-btn>
                </td>
            </tr>
        </tbody>
    </v-table>
  </v-container>
</template>

<script lang="ts" setup>
import { ClusterStateResponse, ClustersService } from "beiboot-api-client";
import { ref, onMounted } from "vue";

let clusterList = ref([] as ClusterStateResponse[]);

const getClusterList = () => {
    ClustersService.clusterListClustersGet().then((res) => {
        clusterList.value = res.items;
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
