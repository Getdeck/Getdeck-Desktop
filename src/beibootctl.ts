import { Command } from "@tauri-apps/api/shell";
import { invoke } from "@tauri-apps/api/tauri";

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

export async function connectCluster(name: string) {
  let res = await invoke("connect_beiboot_ghostunnel", { beibootName: name })
  return res
}

export async function disconnectCluster(name: string) {
  let res = await invoke("disconnect_beiboot_ghostunnel", { beibootName: name })
  return res
}
