//! Config management functions

use std::env;
use std::fs::File;
use std::path::PathBuf;

use std::io::prelude::*;
use std::collections::HashMap;

use yaml_rust::YamlLoader;
use yaml_rust::Yaml;

/// Contains a machine configuration
#[derive(Debug, Clone, Default)]
pub struct MachineConfig {
    /// IP to use
    pub ip: Option<String>,
    /// Port to use
    pub port: Option<u16>,
    /// Username to use
    pub user: Option<String>,
    /// Password to use
    pub pass: Option<String>,
    /// Identity key to use
    pub identity: Option<String>
}

/// Configuration map
pub type ConfigMap = HashMap<String, MachineConfig>;

/// Configuration result
#[derive(Debug)]
pub struct ConfigResult {
    /// Default values for machines
    pub default_values: ConfigMap,
    /// Actual machine values
    pub machine_values: ConfigMap
}

impl MachineConfig {
    /// Merge two configurations together.
    ///
    /// # Arguments
    ///
    /// * `other` - Other machine config to use
    ///
    pub fn merge(&self, other: &MachineConfig) -> MachineConfig {
        let mut config = self.clone();
        
        if other.ip.is_some() {
            config.ip = other.ip.clone();
        }
        
        if other.port.is_some() {
            config.port = other.port;
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
    
    /// Show machine information to stdout.
    ///
    /// # Arguments
    ///
    /// * `machine` - Machine name
    ///
    pub fn show_info(&self, machine: &str) {
        println!("Configuration for `{}`:", machine);
    
        self.ip.as_ref().map(|x| println!("  IP: {}", x));
        self.port.as_ref().map(|x| println!("  Port: {}", x));
        self.user.as_ref().map(|x| println!("  User: {}", x));
        self.pass.as_ref().map(|_| println!("  Pass: *******"));
        self.identity.as_ref().map(|x| println!("  Identity: {}", x));
    }
}

/// Load a configuration from a file path.
///
/// If no path is given, the user configuration path will be used.
///
/// # Arguments
///
/// * `path_to_file` - Path to file (optional)
///
pub fn load_configuration_file(path_to_file: Option<&str>) -> ConfigResult {
    let path_to_file = match path_to_file {
        Some(x) => x.to_string(),
        None => get_user_configuration_path()
    };
    
    debug!("Loading {}...", path_to_file);    
    let mut f = File::open(&path_to_file).expect(&format!("File {} not found.", path_to_file));
    let mut contents = String::new();    
    f.read_to_string(&mut contents).expect("Error while reading file.");
    
    load_configuration_string(&contents)
}

/// Load a configuration from a string.
///
/// # Arguments
///
/// * `contents` - Contents string
///
pub fn load_configuration_string(contents: &str) -> ConfigResult {
    let docs = YamlLoader::load_from_str(contents).unwrap();
    let doc = &docs[0].as_hash().unwrap();
        
    let default_values = doc.get(&Yaml::from_str("defaults")).unwrap();
    let machine_values = doc.get(&Yaml::from_str("machines")).unwrap();
    
    let default_map = extract_definition_keys("", default_values);
    let machine_map = extract_definition_keys("", machine_values);
    let machine_map = apply_machine_configurations(&machine_map, &default_map);
    
    ConfigResult {
        default_values: default_map,
        machine_values: machine_map
    }
}

/// Get the user configuration path
fn get_user_configuration_path() -> String {
    let home_path = match env::home_dir() {
        Some(path) => path.into_os_string().into_string().unwrap(),
        None => "~".to_string()
    };
    
    let full_path: PathBuf = [&home_path, ".pssh", "config.yml"].iter().collect();
    full_path.into_os_string().into_string().unwrap()
}

/// Fetch default values for a machine name
///
/// # Arguments
///
/// * `name` - Machine name
/// * `default_map` - Default values
///
fn fetch_default_values_for_name(name: &str, default_map: &ConfigMap) -> Option<MachineConfig> {
    let default_keys: Vec<String> = default_map.keys().map(|x| x.to_string()).collect();
    let split_parent_names: Vec<&str> = name.split(':').collect();
    
    let mut current_parent: String = "".to_string();
    
    // Fetch global default values
    let mut current_values = if default_keys.contains(&"".to_string()) {
        default_map.get(&"".to_string()).cloned()
    } else {
        None
    };
    
    for parent in split_parent_names {
        if current_parent == "" {
            current_parent = parent.to_string();
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

/// Apply machine configurations on machine map
///
/// # Arguments
///
/// * `machine_map` - Machine configuration map
/// * `default_map` - Default configuration map
///
fn apply_machine_configurations(machine_map: &ConfigMap, default_map: &ConfigMap) -> ConfigMap {
    let mut applied_machines: ConfigMap = HashMap::new();
    
    for k in machine_map.keys() {
        let key = k.clone();
        
        let default_config = fetch_default_values_for_name(k, default_map);
        let machine_config = machine_map.get(k).unwrap();
        
        let machine_applied_config = match default_config {
            Some(x) => x.merge(machine_config),
            None => machine_config.clone()
        };
        
        applied_machines.insert(key, machine_applied_config);
    }
    
    applied_machines
}

/// Extract machine values from YAML
///
/// # Arguments
///
/// * `data` - YAML data
///
fn extract_machine_values(data: &Yaml) -> MachineConfig {
    if data.as_hash().is_none() {
        return Default::default();
    }
    
    let dict_data = data.as_hash().unwrap();
    
    MachineConfig {
        ip: dict_data.get(&Yaml::from_str("ip")).and_then(|x| x.as_str()).map(String::from),
        port: dict_data.get(&Yaml::from_str("port")).and_then(|x| x.as_i64()).map(|x| x as u16),
        user: dict_data.get(&Yaml::from_str("user")).and_then(|x| x.as_str()).map(String::from),
        pass: dict_data.get(&Yaml::from_str("pass")).and_then(|x| x.as_str()).map(String::from),
        identity: dict_data.get(&Yaml::from_str("identity")).and_then(|x| x.as_str()).map(String::from)
    }
}

/// Extract definition keys from YAML
///
/// If the definition is at root level, you should pass the empty string "" as a
/// parent key
///
/// # Arguments
///
/// * `parent_key` - Parent key
/// * `current_yaml` - YAML data
///
fn extract_definition_keys(parent_key: &str, current_yaml: &Yaml) -> ConfigMap {        
    if current_yaml.as_hash().is_none() {
        return HashMap::new();
    }

    let current_dict = current_yaml.as_hash().unwrap();
    let keys = current_dict.keys().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();
    let mut result: ConfigMap = HashMap::new();
    
    if keys.contains(&"$") {
        let values = extract_machine_values(current_dict.get(&Yaml::from_str("$")).unwrap());
        result.insert(parent_key.to_string(), values);
    }
    
    for key in keys {
        if key != "$" {
            if key.contains(':') {
                panic!("Bad character ':' in key: {}", key);
            } 
            
            let current_key = if parent_key == "" {                
                key.to_string()
            } else {
                format!("{}:{}", parent_key, key)
            };
            
            let current_value = current_dict.get(&Yaml::from_str(key)).unwrap();
            let local_results = extract_definition_keys(&current_key, current_value);
            for (k, v) in &local_results {
                result.insert(k.clone(), v.clone());
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn empty_defaults() {
        let str_content = r#"
            defaults:
            machines:
        "#;
        
        let config = load_configuration_string(str_content);
        assert_eq!(config.default_values.keys().len(), 0);
        assert_eq!(config.machine_values.keys().len(), 0);
    }
    
    #[test]
    fn not_empty_defaults() {
        let str_content = r#"
            machines:
            defaults:
                $:
                    user: pouet
                one:
                    two:
                        $:
                            user: pouet
                            port: 5555
                        five:
                            $:
                                port: 5566
                    $:
                        port: 4455
                three:
                    $:
                four:
        "#;
        
        let config = load_configuration_string(str_content);
        assert_eq!(config.machine_values.keys().len(), 0);
        assert_eq!(config.default_values.keys().len(), 5);
        
        assert!(config.default_values.contains_key(""));
        assert!(config.default_values.contains_key("one"));
        assert!(config.default_values.contains_key("three"));
        assert!(config.default_values.contains_key("one:two"));
        assert!(config.default_values.contains_key("one:two:five"));
        assert!(config.default_values.contains_key("four") == false);
    }
    
    #[test]
    #[should_panic]
    fn defaults_error() {
        let str_content = r#"
            machines:
            defaults:
                $:
                    user: pouet
                one:pouet:
                    $:
                        user: hello
        "#;
        
        load_configuration_string(str_content);
    }
    
    #[test]
    fn default_values() {
        let values = hashmap!(
            "".to_string() => MachineConfig {
                user: Some("hello".to_string()),
                port: Some(22),
                ..Default::default()
            },
            "coucou".to_string() => MachineConfig {
                port: Some(23),
                ..Default::default()
            },
            "coucou:hello".to_string() => MachineConfig {
                port: Some(24),
                ..Default::default()
            }
        );
        
        let config = fetch_default_values_for_name(&"toto".to_string(), &values).unwrap();        
        assert_eq!(config.user, Some("hello".to_string()));
        assert_eq!(config.port, Some(22));
        assert_eq!(config.identity, None);
        
        let config = fetch_default_values_for_name(&"coucou".to_string(), &values).unwrap();        
        assert_eq!(config.user, Some("hello".to_string()));
        assert_eq!(config.port, Some(23));
        assert_eq!(config.identity, None);
        
        let config = fetch_default_values_for_name(&"coucou:pouet".to_string(), &values).unwrap();
        assert_eq!(config.user, Some("hello".to_string()));
        assert_eq!(config.port, Some(23));
        
        let config = fetch_default_values_for_name(&"coucou:hello".to_string(), &values).unwrap();
        assert_eq!(config.user, Some("hello".to_string()));
        assert_eq!(config.port, Some(24));
        
        let config = fetch_default_values_for_name(&"coucou:hello:one".to_string(), &values).unwrap();
        assert_eq!(config.user, Some("hello".to_string()));
        assert_eq!(config.port, Some(24));
    }
    
    #[test]
    fn machine_configurations() {
        let defaults = hashmap!(
            "".to_string() => MachineConfig {
                user: Some("hello".to_string()),
                ..Default::default()
            },
            "coucou".to_string() => MachineConfig {
                port: Some(23),
                ..Default::default()
            }
        );
        
        let machines = hashmap!(
            "coucou".to_string() => MachineConfig {
                port: Some(22),
                ..Default::default()
            },
            "coucou:hello".to_string() => MachineConfig {
                ip: Some("127.0.0.1".to_string()),
                ..Default::default()
            }
        );
        
        let configured_machines = apply_machine_configurations(&machines, &defaults);
        let m_coucou = configured_machines.get(&"coucou".to_string()).unwrap(); 
        let m_coucou_hello = configured_machines.get(&"coucou:hello".to_string()).unwrap(); 
        
        assert_eq!(m_coucou.port, Some(22));
        assert_eq!(m_coucou.user, Some("hello".to_string()));
        
        assert_eq!(m_coucou_hello.user, Some("hello".to_string()));
        assert_eq!(m_coucou_hello.port, Some(23));
        assert_eq!(m_coucou_hello.ip, Some("127.0.0.1".to_string()));
    }
}
