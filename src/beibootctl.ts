import { Command } from "@tauri-apps/api/shell";
import { invoke } from "@tauri-apps/api/tauri";
import { useAppStore } from "./store/app";

export async function getVersion() {
  return "1.0.0";
}

export async function createCluster(name: string) {
  console.log("Ich laufe");
  console.log(name);

  return "Created cluster " +  name;
}

export async function deleteCluster(name: string) {
  return "Deleted cluster " +  name;
}

export async function listCluster() {
  const output = "No Beiboots";
  console.log(output);
}

export async function connectCluster(name: string, portMapping: any, ca: string, clCert: string, clKey: string) {
  let res = await invoke("connect_beiboot_ghostunnel", {
    beibootName: name,
    ports: portMapping,
    ca,
    clCert,
    clKey
  })
  return res
}

export async function writeKubeconfig(name: string, kubeconfig: string): Promise<string> {
  let res: string = await invoke("write_kubeconfig", {
    beibootName: name,
    kubeconfig
  })
  return res
}

export async function disconnectCluster(name: string) {
  let res = await invoke("disconnect_beiboot_ghostunnel", { beibootName: name })
  return res
}

export async function startOAuthServer() {
  let res = await invoke("start_server", {})
  return res
}

export async function checkRunningConnects() {
  const appStore = useAppStore();

  try {
    let res: string[] = await invoke("check_running_connects", {})
    if (res.length > 0) {
      appStore.connection.clusterName = res[0]
      appStore.connection.connected = true;
    } else {
      appStore.connection.clusterName = ""
      appStore.connection.connected = false;
    }
    return res
  } catch {
    appStore.connection.clusterName = ""
    appStore.connection.connected = false;
    console.log("no docker client here.")
    return "Docker Client unresponsive."
  }
}
export async function checkDockerEngine() {
  let res: string = await invoke("check_docker_engine", {})
  return res
}
