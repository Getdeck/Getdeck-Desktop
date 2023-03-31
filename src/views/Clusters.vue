<template>
  <v-container class="fill-height">
    <h1>Clusters</h1>
    <v-table>
        <thead>
            <tr>
                <th>Name</th>
                <th>State</th>
                <th>Actions</th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="cluster in clusterList" :key="name">
                <td>{{ cluster.name }}</td>
                <td>{{ cluster.state }}</td>
                <td>
                    <v-btn class="mb-1 mt-3 mr-1" min-width="110" variant="outlined">Connect</v-btn>
                    <v-btn class="mb-1 mt-3" min-width="110" variant="outlined">Edit</v-btn> <br>
                    <v-btn class="mb-3 mr-1" @click="handleClusterDelete(cluster.name)" min-width="110" variant="outlined">Delete</v-btn>
                    <v-btn class="mb-3" min-width="110" variant="outlined">Shelf</v-btn>
                </td>
            </tr>
        </tbody>
    </v-table>
  </v-container>
</template>

<script lang="ts" setup>
import { ClustersService, ClusterStateResponse } from "beiboot-api-client";
import { ref } from 'vue';

const clusterList = ref();

ClustersService.clusterListClustersGet().then((response) => {
    clusterList.value = response.items;
})

function handleClusterDelete(clusterName: string) {
    ClustersService.clusterDeleteClustersClusterNameDelete(clusterName).then((response) => {
        console.log(response);
    })
}

</script>
