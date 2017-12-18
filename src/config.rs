//! Config

use std::env;
use std::fs::File;
use std::path::PathBuf;

use std::io::prelude::*;
use std::collections::HashMap;

use yaml_rust::YamlLoader;
use yaml_rust::Yaml;

#[derive(Debug)]
struct MachineConfig {
    ip: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    pass: Option<String>,
    identity: Option<String>
}

type ConfigMap = HashMap<String, MachineConfig>;

fn get_user_configuration_path() -> String {
    let home_path = match env::home_dir() {
        Some(path) => path.into_os_string().into_string().unwrap(),
        None => "~".to_owned()
    };
    
    let full_path: PathBuf = [&home_path, ".pssh", "config.yml"].iter().collect();
    full_path.into_os_string().into_string().unwrap()
}

pub fn load_configuration_file(path_to_file: Option<String>) {
    let path_to_file = match path_to_file {
        Some(x) => x,
        None => get_user_configuration_path()
    };
    
    println!("Loading {}...", path_to_file);
    
    let mut f = File::open(&path_to_file).expect(&format!("File {} not found.", path_to_file));
    let mut contents = String::new();
    
    f.read_to_string(&mut contents).expect("Error while reading file.");
    
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0].as_hash().unwrap();
        
    let default_values = doc.get(&Yaml::from_str("defaults")).unwrap();
    let machine_values = doc.get(&Yaml::from_str("machines")).unwrap();
    
    println!("{:?}", default_values);
    println!("{:?}", machine_values);
    
    handle_default_values(&default_values);
}

fn handle_default_values(values: &Yaml) {
    _extract_definition_keys("", values);
}

fn _extract_machine_values(data: &Yaml) -> MachineConfig {
    let dict_data = data.as_hash().unwrap();
    
    MachineConfig {
        ip: dict_data.get(&Yaml::from_str("ip")).and_then(|x| x.as_str()).map(String::from),
        port: dict_data.get(&Yaml::from_str("port")).and_then(|x| x.as_i64()).map(|x| x as u16),
        user: dict_data.get(&Yaml::from_str("user")).and_then(|x| x.as_str()).map(String::from),
        pass: dict_data.get(&Yaml::from_str("pass")).and_then(|x| x.as_str()).map(String::from),
        identity: dict_data.get(&Yaml::from_str("identity")).and_then(|x| x.as_str()).map(String::from)
    }
}

fn _extract_definition_keys(parent_key: &str, current_yaml: &Yaml) {
    let current_dict = current_yaml.as_hash().unwrap();    
    let keys = current_dict.keys().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();
    let mut result: ConfigMap = HashMap::new();
    
    if keys.contains(&"_values") {
        let values = _extract_machine_values(current_dict.get(&Yaml::from_str("_values")).unwrap());
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
            
            println!("{}", current_key);
            
            _extract_definition_keys(&current_key, &current_value);
        }
    }

    println!("{:?}", result);
}

fn get_machine_configuration() {
    
}

