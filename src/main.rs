use serde::{Deserialize, Serialize};
use tomlreadwr::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpcuaConfig {
    pub source_type: String,
    pub enabled: bool,
    pub host: String,
    pub collection_duration: u32,
    pub machine_prefix: String,
    pub machine_ip: String,
    pub file_type: String,
    pub response_type: String,
    pub authtype: String,
    pub collection_interval_seconds: u32,
    pub namespace: u32,
    pub node_variance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpcuaConf {
    pub authtype: String,
    pub collection_duration: u32,
    pub collection_interval_seconds: u32,
    pub enabled: bool,
    pub file_type: String,
    pub host: String,
    pub machine_ip: String,
    pub machine_prefix: String,
    pub namespace: u32,
    pub node_variance: Vec<String>,
    pub response_type: String,
    pub source_type: String,
}

fn main() -> anyhow::Result<()> {
    // Load
    let config = TomlConfig::load("./sources.conf")?;

    // Read
    println!("Read:");
    println!("{:?}", config.get("sources.opcua_machine1"));
    let opcua = config.get_of_type::<OpcuaConf>("sources.opcua_machine1");
    println!("OPCUA CONFIG: {opcua:#?}");

    Ok(())
}
