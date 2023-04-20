#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use beiboot_desktop::connection::{get_connector_context, PortMapping, TLSFiles};
mod util;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect_beiboot_ghostunnel, disconnect_beiboot_ghostunnel])    
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn connect_beiboot_ghostunnel(beiboot_name: String, port: PortMapping, ca: &str, cl_cert: &str, cl_key: &str) -> Result<String, String> {
    let connector = get_connector_context(&beiboot_name, "GhostunnelDocker");
    let ports = [port];
    let ca_cert_path = util::write_tls_files(beiboot_name.clone(), ca, "ca.crt").unwrap();
    let client_cert_path = util::write_tls_files(beiboot_name.clone(), cl_cert, "client.crt").unwrap();
    let client_key_path = util::write_tls_files(beiboot_name.clone(), cl_key, "client.key").unwrap();

    let tls = TLSFiles {
        ca_cert_path: &ca_cert_path,
        client_cert_path: &client_cert_path,
        client_key_path: &client_key_path,
    };

    match connector.connect(&ports, &tls) {
        Ok(_) => Ok("Cluster connected successfully".into()),
        Err(why) => {
            println!("{}", why);
            Err(format!("{}", why).into())
        }
    }
}

#[tauri::command]
fn disconnect_beiboot_ghostunnel(beiboot_name: String) -> Result<String, String> {
    let connector = get_connector_context(&beiboot_name, "GhostunnelDocker");
    match connector.disconnect() {
        Ok(_) => Ok("Cluster disconnected successfully".into()),
        Err(why) => {
            println!("{}", why);
            Err(format!("{}", why).into())
        }
    }
}
