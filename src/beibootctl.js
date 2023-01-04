import { Command } from "@tauri-apps/api/shell";
export async function getVersion() {
    const command = Command.sidecar("bin/beibootctl", ["version"]);
    const output = await command.execute();
    console.log(output);
    if (output.code == 0) {
        // TODO parse version string
        return output.stdout;
    }
    console.log(output.stderr);
    return "";
}
export async function createCluster(name) {
    console.log("Ich laufe");
    console.log(name);
    const command = Command.sidecar("bin/beibootctl", [
        "cluster",
        "create",
        name,
    ]);
    return command.execute();
}
export async function deleteCluster(name) {
    const command = Command.sidecar("bin/beibootctl", [
        "cluster",
        "delete",
        name,
    ]);
    return command.execute();
}
export async function listCluster() {
    const command = Command.sidecar("bin/beibootctl", ["cluster", "list"]);
    const output = await command.execute();
    console.log(output);
}
