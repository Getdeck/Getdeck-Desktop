import { Command } from '@tauri-apps/api/shell'

export async function getVersion() {
  const command = Command.sidecar('binaries/beibootctl', ['version'])
  const output = await command.execute();
  console.log(output)
  if (output.code == 0) {
    // TODO parse version string
    return output.stdout
  }
  console.log(output.stderr)
  return ""
}

export async function createCluster(name:string) {
  console.log("Ich laufe")
  console.log(name)
  const command = Command.sidecar('binaries/beibootctl', ['cluster', 'create', name]);
  return command.execute();
}

export async function listCluster() {
  const command = Command.sidecar('binaries/beibootctl', ['cluster', 'list']);
  const output = await command.execute();
  console.log(output)
}
