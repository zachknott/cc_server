use std::net::IpAddr;

use local_ip_address::local_ip;
use serde::{Serialize, Deserialize};

use serde_json;
use hostname;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    hostname: String,
    ip: IpAddr
}

impl ClientConfig {
    pub fn new() -> ClientConfig {
        let hostname = hostname::get().unwrap().into_string().unwrap();
        let ip = local_ip().unwrap();

        ClientConfig {
            hostname,
            ip
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_config(config_string: String) -> ClientConfig {
        let response: ClientConfig = serde_json::from_str(&config_string).unwrap();
        response
    }
}