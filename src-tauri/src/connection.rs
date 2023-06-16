use std::{error::Error, fmt};
use serde::Deserialize;

pub mod ghostunnel;

pub fn get_connector_context<'a>(beiboot_name: &'a str, strategy: &str) -> ConnectorContext<'a> {
    let imp_strategy = match strategy {
        "GhostunnelDocker" => ghostunnel::GhostunnelDocker,
        _ => ghostunnel::GhostunnelDocker,
    };
    ConnectorContext {
        connector: Box::new(imp_strategy),
        name: beiboot_name,
    }
}

#[derive(Debug)]
pub struct ConnectError {
    details: String,
}

impl ConnectError {
    fn new(msg: &str) -> ConnectError {
        ConnectError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ConnectError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct ConnectorContext<'a> {
    pub connector: Box<dyn Connector>,
    pub name: &'a str,
}

impl<'a> ConnectorContext<'a> {
    pub fn connect(&self, ports: &[PortMapping], mtls: &TLSFiles) -> Result<(), ConnectError> {
        println!("Connecting to Beiboot {}", self.name);
        let result = self.connector.establish(self.name, ports, mtls);
        println!("Connection to Beiboot {} established", self.name);
        result
    }
    pub fn disconnect(&self) -> Result<(), ConnectError> {
        println!("Disconnect preamble");
        let result = self.connector.terminate(self.name);
        println!("Disconnect postamble");
        result
    }
    pub fn check_running(&self) -> Result<Vec<String>, ConnectError> {
        
        self.connector.check_running()
    }
}

#[derive(Deserialize)]
pub struct PortMapping<'a> {
    pub target: u16,
    pub endpoint: &'a str,
}

pub struct TLSFiles<'a> {
    pub ca_cert_path: &'a str,
    pub client_key_path: &'a str,
    pub client_cert_path: &'a str,
}

pub trait Connector {
    fn establish(
        &self,
        name: &str,
        ports: &[PortMapping],
        mtls: &TLSFiles,
    ) -> Result<(), ConnectError>;
    fn check_running(&self) -> Result<Vec<String>, ConnectError>;
    fn terminate(&self, name: &str) -> Result<(), ConnectError>;
}
