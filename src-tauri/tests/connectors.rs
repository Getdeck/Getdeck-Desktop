#[cfg(test)]
mod ghostunnel_docker_tests {
    use beiboot_desktop::connection::{get_connector_context, PortMapping, TLSFiles};

    //FIXME: This test fails after the first run because of a container name conflict.
    #[test]
    fn test_a_connect_ghostunnel_docker() {
        let connector = get_connector_context("test", "GhostunnelDocker");

        let ports = vec![PortMapping {
            target: 6443,
            endpoint: "34.88.204.180:31477",
        }, PortMapping {
            target: 8080,
            endpoint: "34.88.204.180.31477"
        }];
        let ca_cert_path = shellexpand::tilde("~/.getdeck/test/mtls/ca.crt").into_owned();
        let client_cert_path = shellexpand::tilde("~/.getdeck/test/mtls/client.crt").into_owned();
        let client_key_path = shellexpand::tilde("~/.getdeck/test/mtls/client.key").into_owned();

        let tls = TLSFiles {
            ca_cert_path: &ca_cert_path,
            client_cert_path: &client_cert_path,
            client_key_path: &client_key_path,
        };

        let res = match connector.connect(&ports, &tls) {
            Ok(_) => 0,
            Err(why) => {
                println!("{}", why);
                1
            }
        };
        assert_eq!(0, res)
    }

    #[test]
    fn test_c_disconnect_ghostunnel_docker() {
        let connector = get_connector_context("test", "GhostunnelDocker");
        let res = match connector.disconnect() {
            Ok(_) => 0,
            Err(why) => {
                println!("{}", why);
                1
            }
        };
        assert_eq!(0, res)
    }
    #[test]
    fn test_b_check_running() {
        let connector = get_connector_context("test", "GhostunnelDocker");
        let res = match connector.check_running() {
            Ok(container_summary) => {
                println!("{:?}", container_summary);
                0
            },
            Err(why) => {
                println!("{}", why);
                1
            }
        };
    }
}
