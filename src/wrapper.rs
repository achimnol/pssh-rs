//! Shell wrappers

use std::process::Command;

use config::MachineConfig;

pub fn ping(ip: &String) {
    let mut command = Command::new("ping");
    command.arg(ip);      
        
    debug!("Executing {}", format!("{:?}", command));
        
    let mut child = command.spawn().expect("Failed to execute ping");
    child.wait().expect("Failed to wait on child");
}

pub fn scp(config: &MachineConfig, source: &str, destination: &str) {
    let mut child = Command::new("scp");
    
    if config.identity.is_some() {
        child.args(&["-i", &(config.identity.as_ref().unwrap())]);
    }
    
    if config.port.is_some() {
        child.args(&["-P", &(config.port.as_ref().unwrap().to_string())]);
    } else {
        child.args(&["-P", "22"]);
    }
    
    child.args(&[source]);
    
    let user_path: String;
    if config.user.is_some() {
        user_path = format!("{}@{}:{}",
            &config.user.as_ref().unwrap(),
            &config.ip.as_ref().unwrap(),
            destination
        );
    } else {
        user_path = format!("{}:{}",
            &config.ip.as_ref().unwrap(),
            destination
        );
    }
    
    child.arg(&user_path);

    let mut child_proc = child.spawn().expect("Failed to execute ping");
    child_proc.wait().expect("Failed to wait on child");
}

pub fn ssh(config: &MachineConfig, user: Option<&str>, tmux: bool) {
    let mut child = Command::new("ssh");
    
    if config.identity.is_some() {
        child.args(&["-i", &(config.identity.as_ref().unwrap())]);
    }
    
    if config.port.is_some() {
        child.args(&["-p", &(config.port.as_ref().unwrap().to_string())]);
    } else {
        child.args(&["-p", "22"]);
    }
    
    let user_name: Option<&str>;
    if user.is_none() {
        if !config.user.is_none() {
            user_name = config.user.as_ref().map(|x| &x[..]);
        } else {
            user_name = None
        }
    } else {
        user_name = user;
    }
    
    let user_path: String;
    if user_name.is_some() {
        user_path = format!("{}@{}",
            user_name.unwrap(),
            config.ip.as_ref().unwrap()
        );
    } else {
        user_path = format!("{}",
            config.ip.as_ref().unwrap()
        );
    }
    
    child.arg(&user_path);
    
    if tmux {
        child.arg("tmux attach || tmux new");
    }
    
    let mut child_proc = child.spawn().expect("Failed to execute ping");
    child_proc.wait().expect("Failed to wait on child");
}