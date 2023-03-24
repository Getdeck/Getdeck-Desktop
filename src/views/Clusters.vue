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
                    <v-btn class="mb-3 mr-1" min-width="110" variant="outlined">Delete</v-btn>
                    <v-btn class="mb-3" min-width="110" variant="outlined">Shelf</v-btn>
                </td>
            </tr>
        </tbody>
    </v-table>
  </v-container>
</template>

<script lang="ts" setup>
import { OpenAPI } from "@/client/index";

const client = new OpenAPI({
    basePath: "https://api.beiboot.unikube.io",
})

client.

console.log(clusterList)

const clusterListInner = [
    {
        name: "myhost-myuser-cluster1",
        state: "READY",
        k8sVersion: "1.25.1",
        nodeCount: 3,
        nodeCpu: 2,
        nodeMemoryRequest: "2Gi",
        nodeMemoryLimit: "4Gi",
        ports: [
            {
                cluster: 80,
                host: 8080,
            },
            {
                cluster: 443,
                host: 8443
            }
        ]
    },
    {
        name: "myhost-myuser-cluster2",
        state: "READY",
        k8sVersion: "1.25.1",
        nodeCount: 3,
        nodeCpu: 2,
        nodeMemoryRequest: "2Gi",
        nodeMemoryLimit: "4Gi",
        ports: [
            {
                cluster: 80,
                host: 8080,
            },
            {
                cluster: 443,
                host: 8443
            }
        ]
    }
]

</script>
