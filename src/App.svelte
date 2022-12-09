<script>
import { getVersion, createCluster } from './beibootctl'

let promise = getVersion();
let clusterName;
let handlerPromise;
let promiseRunning = false;
let error = "";

function createClusterHandler() {
  error = "";
  promiseRunning = true;
  console.log(clusterName)
  if(clusterName) {
    handlerPromise = createCluster(clusterName).then( (output) => { 
      promiseRunning = false; 
      console.log(output);
      if (output.code !== 0) {
        error = output.stderr;
      }
    });
  }
}</script>

<div id="app">
<img class="logo-light" src="beiboot-logo-light.png">
<img class="logo-dark" src="beiboot-logo-dark.svg">


  {#await promise}
    <p>Loading…</p>
    {:then version}
    <p>{version}</p>
  {/await}

<input bind:value={clusterName}>

<button on:click={createClusterHandler} disabled={promiseRunning}>Create Cluster</button>
<button on:click={createClusterHandler}>Delete Cluster</button>
{#await handlerPromise}
  <p>Cluster gets created</p>
  {:then clusterOutput}
    {#if error }
      <p>{error}</p>
    {:else}
      <p>Cluster created successfully</p>
    {/if}

{/await}

</div>

