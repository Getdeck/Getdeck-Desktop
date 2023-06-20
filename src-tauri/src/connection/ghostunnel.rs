use bollard::container::{Config, CreateContainerOptions, StartContainerOptions, ListContainersOptions, RemoveContainerOptions};
use bollard::service::{HostConfig, PortBinding, PortMap, RestartPolicy, RestartPolicyNameEnum};
use bollard::Docker;

use std::collections::HashMap;
use futures_util::TryStreamExt;

use crate::connection::{Connector, PortMapping, TLSFiles};

use super::ConnectError;

pub struct GhostunnelDocker;

static GHOSTUNNEL_IMAGE: &str = "ghostunnel/ghostunnel:v1.7.1";

impl Connector for GhostunnelDocker {
    fn establish(&self, name: &str, ports: &[PortMapping], mtls: &TLSFiles) -> Result<(), ConnectError> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let docker = match Docker::connect_with_socket_defaults() {
                    Err(why) => return Err(ConnectError::new(format!("Docker error: {}", why).as_str())),
                    Ok(docker) => docker,
                };                

                let version = docker.version().await.unwrap();
                let platform = format!("linux/{}", version.arch.unwrap());

                for port in ports {
                    let container_name = format!(
                        "getdeck-beiboot-{name}-{local_port}",
                        name = name,
                        local_port = port.target
                    );
                    let fcmd = format!(
                        "client --listen 0.0.0.0:{local_port} --unsafe-listen --target {endpoint} --cert /crt/client.crt --key /crt/client.key --cacert /crt/ca.crt",
                        local_port = port.target,
                        endpoint = port.endpoint
                    );
                    let cmd: Vec<&str> = fcmd.split(' ').collect();
                    let options = Some(CreateContainerOptions {
                        name: container_name.as_str(),
                        platform: Some(&platform),
                    });
                    let labels = HashMap::from([
                        ("beiboot.getdeck.dev/name", name)
                    ]);

                    let iport = port.target.to_string();
                    let exposed_ports = HashMap::from(
                            [(iport.as_str(), HashMap::from([]))]
                        );

                    let port_map: PortMap = HashMap::from(
                        [
                            (iport.clone(),
                                Some(vec![
                                    PortBinding {host_ip: Some("127.0.0.1".to_string()), host_port: Some(iport.clone())}
                                ])
                            )
                        ]);
                    let cacrt = format!("{}:{}", mtls.ca_cert_path, "/crt/ca.crt");
                    let clientcrt = format!("{}:{}", mtls.client_cert_path, "/crt/client.crt");
                    let clientkey = format!("{}:{}", mtls.client_key_path, "/crt/client.key");
                    let bind_mounts = vec![cacrt, clientcrt, clientkey];

                    let hostconfig = HostConfig {
                        auto_remove: Some(false),
                        restart_policy: Some(RestartPolicy { name: Some(RestartPolicyNameEnum::UNLESS_STOPPED), maximum_retry_count: None}),
                        binds: Some(bind_mounts),
                        port_bindings: Some(port_map),
                        ..Default::default()
                    };

                    let _ghostunnel_image = docker.create_image(
                        Some(bollard::image::CreateImageOptions {
                            from_image: GHOSTUNNEL_IMAGE,
                            ..Default::default()
                        }), 
                        None,
                        None
                    ).try_collect::<Vec<_>>().await;

                    let ghostunnel_config = Config {
                        image: Some(GHOSTUNNEL_IMAGE),
                        cmd: Some(cmd),
                        exposed_ports: Some(exposed_ports),
                        host_config: Some(hostconfig),
                        labels: Some(labels),
                        ..Default::default()
                    };
                    
                    if let Err(why) = docker.create_container(options, ghostunnel_config).await {
                        return Err(ConnectError::new(format!("Error creating container: {}", why).as_str())) 
                    }

                    println!(
                        "Creating forwarding from {} to {}",
                        port.target, port.endpoint
                    );
                    println!(
                        "with CA {}, Client {}, Key {}",
                        mtls.ca_cert_path, mtls.client_cert_path, mtls.client_key_path
                    );
                    let sval = docker
                        .start_container(container_name.as_str(), None::<StartContainerOptions<String>>)
                        .await;

                    match sval {
                        Ok(sval) => sval,
                        Err(why) => return Err(ConnectError::new(format!("Could not start container: {}", why).as_str())),
                    }
                }
                Ok(())
            })
            
    }

    fn terminate(&self, name: &str) -> Result<(), ConnectError> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let docker = match Docker::connect_with_socket_defaults() {
                    Err(why) => return Err(ConnectError::new(format!("Docker error: {}", why).as_str())),
                    Ok(docker) => docker,
                }; 


                let name_label = if name.is_empty() { "beiboot.getdeck.dev/name".to_string() } else { format!("beiboot.getdeck.dev/name={name}", name=name) };


                let filters = HashMap::from([
                    ("label", vec![name_label.as_str()])
                ]);

                let options = Some(ListContainersOptions{
                    filters,
                    ..Default::default()
                });

                let remove_options = Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                });
                
                

                let rcontainers = docker.list_containers(options).await;
                let containers = match rcontainers {
                    Ok(containers) => containers,
                    Err(why) => return Err(ConnectError::new(format!("Could not find containers: {}", why).as_str())),
                };
                for container in containers{
                    match container.id {
                        None => None,
                        Some(i) => Some(docker.remove_container(i.as_str(), remove_options).await),
                    };
                }




                Ok(())
            })

    }

    fn check_running(&self) -> Result<Vec<String>, ConnectError> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let docker = match Docker::connect_with_socket_defaults() {
                    Err(why) => return Err(ConnectError::new(format!("Docker error: {}", why).as_str())),
                    Ok(docker) => docker,
                };

                let name_label = "beiboot.getdeck.dev/name".to_string();

                let filters = HashMap::from([
                    ("label", vec![name_label.as_str()])
                ]);

                let options = Some(ListContainersOptions{
                    filters,
                    ..Default::default()
                });

                let mut result = vec![];

                let rcontainers = docker.list_containers(options).await;
                match rcontainers {
                    Ok(containers) => {
                        for container in &containers {
                            match &container.labels.clone().expect("Could not get labels.").get("beiboot.getdeck.dev/name") {
                                None => return Err(ConnectError::new(format!("Could not find containers: {}", "No name label").as_str())),
                                Some(name) => result.push(name.to_string()),
                            }
                        }
                        Ok(result)
                    },
                    Err(why) => return Err(ConnectError::new(format!("Could not find containers: {}", why).as_str())),
                }
            })
    }
}
