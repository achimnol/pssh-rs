//! Config

use std::env;
use std::fs::File;
use std::path::PathBuf;

use std::io::prelude::*;
use std::collections::HashMap;

use yaml_rust::YamlLoader;
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct MachineConfig {
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub identity: Option<String>
}

pub type ConfigMap = HashMap<String, MachineConfig>;

impl MachineConfig {
    pub fn merge(&self, other: &MachineConfig) -> MachineConfig {
        let mut config = self.clone();
        
        if other.ip.is_some() {
            config.ip = other.ip.clone();
        }
        
        if other.port.is_some() {
            config.port = other.port.clone();
        }
        
        if other.user.is_some() {
            config.user = other.user.clone();
        }
        
        if other.pass.is_some() {
            config.pass = other.pass.clone();
        }
        
        if other.identity.is_some() {
            config.identity = other.identity.clone();
        }
        
        config
    }
    
    pub fn show_info(&self, machine: &str) {
        println!("Configuration for `{}`:", machine);
        
        if self.ip.is_some() {
            println!("  IP: {}", self.ip.as_ref().unwrap());
        }
        if self.port.is_some() {
            println!("  Port: {}", self.port.as_ref().unwrap());
        }
        if self.user.is_some() {
            println!("  User: {}", self.user.as_ref().unwrap());
        }
        if self.pass.is_some() {
            println!("  Pass: *******");
        }
        if self.identity.is_some() {
            println!("  Identity: {}", self.identity.as_ref().unwrap());
        }
    }
}

pub fn load_configuration_file(path_to_file: Option<String>) -> ConfigMap {
    let path_to_file = match path_to_file {
        Some(x) => x,
        None => get_user_configuration_path()
    };
    
    debug!("Loading {}...", path_to_file);
    
    let mut f = File::open(&path_to_file).expect(&format!("File {} not found.", path_to_file));
    let mut contents = String::new();
    
    f.read_to_string(&mut contents).expect("Error while reading file.");
    
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0].as_hash().unwrap();
        
    let default_values = doc.get(&Yaml::from_str("defaults")).unwrap();
    let machine_values = doc.get(&Yaml::from_str("machines")).unwrap();
    
    let default_map = handle_values(&default_values);
    let machine_map = handle_values(&machine_values);
    
    apply_machine_configurations(&machine_map, &default_map)
}

fn get_user_configuration_path() -> String {
    let home_path = match env::home_dir() {
        Some(path) => path.into_os_string().into_string().unwrap(),
        None => "~".to_owned()
    };
    
    let full_path: PathBuf = [&home_path, ".pssh", "config.yml"].iter().collect();
    full_path.into_os_string().into_string().unwrap()
}

fn fetch_default_values_for_name(name: &String, default_map: &ConfigMap) -> Option<MachineConfig> {
    let default_keys: Vec<String> = default_map.keys().map(|x| x.to_owned()).collect();
    let split_parent_names: Vec<&str> = name.split(':').collect();
    
    let mut current_parent: String = "".to_owned();
    let mut current_values: Option<MachineConfig> = None;
    
    // Fetch global default values
    if default_keys.contains(&"".to_owned()) {
        current_values = default_map.get(&"".to_owned()).map(|x| x.clone());
    }
    
    for parent in split_parent_names {
        if current_parent == "" {
            current_parent = parent.to_owned();
        } else {
            current_parent = format!("{}:{}", current_parent, parent);
        }
        
        if default_keys.contains(&current_parent) {
            let parent_values = default_map.get(&current_parent).unwrap().clone();
            current_values = match current_values {
                Some(x) => Some(x.merge(&parent_values)),
                None => Some(parent_values)
            };
        }
     }
    
    current_values
}

fn handle_values(values: &Yaml) -> ConfigMap {
    extract_definition_keys("", values)
}

fn apply_machine_configurations(machine_map: &ConfigMap, default_map: &ConfigMap) -> ConfigMap {
    let mut applied_machines: ConfigMap = HashMap::new();
    
    for k in machine_map.keys() {
        let key = k.clone();
        
        let default_config = fetch_default_values_for_name(k, &default_map);
        let machine_config = machine_map.get(k).unwrap();
        
        let machine_applied_config = match default_config {
            Some(x) => machine_config.merge(&x),
            None => machine_config.clone()
        };
        
        applied_machines.insert(key, machine_applied_config);
    }
    
    applied_machines
}

fn extract_machine_values(data: &Yaml) -> MachineConfig {
    let dict_data = data.as_hash().unwrap();
    
    MachineConfig {
        ip: dict_data.get(&Yaml::from_str("ip")).and_then(|x| x.as_str()).map(String::from),
        port: dict_data.get(&Yaml::from_str("port")).and_then(|x| x.as_i64()).map(|x| x as u16),
        user: dict_data.get(&Yaml::from_str("user")).and_then(|x| x.as_str()).map(String::from),
        pass: dict_data.get(&Yaml::from_str("pass")).and_then(|x| x.as_str()).map(String::from),
        identity: dict_data.get(&Yaml::from_str("identity")).and_then(|x| x.as_str()).map(String::from)
    }
}

fn extract_definition_keys(parent_key: &str, current_yaml: &Yaml) -> ConfigMap {
    let current_dict = current_yaml.as_hash().unwrap();    
    let keys = current_dict.keys().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();
    let mut result: ConfigMap = HashMap::new();
    
    if keys.contains(&"_values") {
        let values = extract_machine_values(current_dict.get(&Yaml::from_str("_values")).unwrap());
        result.insert(parent_key.to_owned(), values);
    }
    
    for key in keys {
        if key != "_values" {
            if key.contains(':') {
                panic!("Bad character ':' in key: {}", key);
            } 
            
            let current_key: String;
            if parent_key == "" {
                current_key = key.to_owned();
            } else {
                current_key = format!("{}:{}", parent_key, key);
            }
            
            let current_value = current_dict.get(&Yaml::from_str(key)).unwrap();
            let local_results = extract_definition_keys(&current_key, &current_value);
            for (k, v) in local_results.iter() {
                result.insert(k.clone(), v.clone());
            }
        }
    }

    result
}
