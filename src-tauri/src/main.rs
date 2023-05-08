#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use beiboot_desktop::connection::{get_connector_context, PortMapping, TLSFiles};
use tauri_plugin_oauth::start;
use tauri::{utils::config::AppUrl, window::WindowBuilder, WindowUrl};

mod util;

fn main() {
    let _guard = sentry::init(("https://a64f388330914f08b0a015e6068dac3d@o146863.ingest.sentry.io/4505356777357312", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    let mut context = tauri::generate_context!();
    let mut builder = tauri::Builder::default();

    let port = portpicker::pick_unused_port().expect("failed to find an available port");
    #[cfg(dev)]
    {
        let port = 5173;
    }
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());

    builder = builder
        .invoke_handler(tauri::generate_handler![connect_beiboot_ghostunnel, disconnect_beiboot_ghostunnel, write_kubeconfig, cleanup, start_server, check_running_connects])
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |app| {
            WindowBuilder::new(
                app,
                "main".to_string(),
                if cfg!(dev) {
                    Default::default()
                } else {
                    window_url
                }
                )
                .fullscreen(false)
                .inner_size(1200.0, 800.0)
                .title("Getdeck Desktop")
                .build()?;
            Ok(())
        })
    .on_window_event(|event| {
        match event.event() {
            tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed { .. } => {
                get_connector_context("", "GhostunnelDocker").disconnect().unwrap();
            }
            _ => {}
        }
    });

    builder
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
async fn start_server(window: tauri::Window) -> Result<u16, String> {
    let tauri_url = window.url();
    start(move |url| {
        let params = url.split("#").collect::<Vec<&str>>();
        window.eval(format!("window.location.replace('{}#{}')", tauri_url, params[1]).as_str()).unwrap();
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn connect_beiboot_ghostunnel(beiboot_name: String, ports: Vec<PortMapping>, ca: &str, cl_cert: &str, cl_key: &str) -> Result<String, String> {
    let connector = get_connector_context(&beiboot_name, "GhostunnelDocker");
    let ca_cert_path = util::write_conf_file(beiboot_name.clone(), ca, "ca.crt").unwrap();
    let client_cert_path = util::write_conf_file(beiboot_name.clone(), cl_cert, "client.crt").unwrap();
    let client_key_path = util::write_conf_file(beiboot_name.clone(), cl_key, "client.key").unwrap();

    let tls = TLSFiles {
        ca_cert_path: &ca_cert_path,
        client_cert_path: &client_cert_path,
        client_key_path: &client_key_path,
    };

    match connector.connect(&ports, &tls) {
        Ok(_) => Ok("Cluster connected successfully".into()),
        Err(why) => {
            println!("{}", why);
            Err(format!("{}", why))
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
            Err(format!("{}", why))
        }
    }
}

#[tauri::command]
fn write_kubeconfig(beiboot_name: String, kubeconfig: String) -> Result<String, String> {
    match util::write_conf_file(beiboot_name, &kubeconfig, "kubeconfig.yaml") {
        Ok(path) => Ok(path),
        Err(why) => {
            println!("{}", why);
            Err(why)
        }
    }
}

#[tauri::command]
fn cleanup(beiboot_name: String) -> Result<(), String> {
    match util::cleanup(beiboot_name) {
        Ok(_) => Ok(()),
        Err(why) => {
            println!("{}", why);
            Err(why)
        }
    }
}

#[tauri::command]
fn check_running_connects() -> Result<Vec<String>, String> {
    let connector = get_connector_context("any", "GhostunnelDocker");
    match connector.check_running() {
        Ok(running) => Ok(running),
        Err(why) => {
            println!("{}", why);
            Err(format!("{}", why))
        }
    }
}
