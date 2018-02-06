//! Shell wrappers

use std::process::Command;

use config::MachineConfig;

pub fn ping(ip: &str) {
    let mut command = Command::new("ping");
    command.arg(ip);      
        
    debug!("Executing {}", format!("{:?}", command));
        
    let mut child = command.spawn().expect("Failed to execute ping");
    child.wait().expect("Failed to wait on child");
}

pub fn scp(config: &MachineConfig, source: &str, destination: &str) {
    let mut command = Command::new("scp");
    
    if config.identity.is_some() {
        command.args(&["-i", config.identity.as_ref().unwrap()]);
    }
    
    if config.port.is_some() {
        command.args(&["-P", &(config.port.as_ref().unwrap().to_string())]);
    } else {
        command.args(&["-P", "22"]);
    }
    
    command.args(&[source]);
    
    let user_path = if config.user.is_some() {
        format!("{}@{}:{}",
            &config.user.as_ref().unwrap(),
            &config.ip.as_ref().unwrap(),
            destination
        )
    } else {
        format!("{}:{}",
            &config.ip.as_ref().unwrap(),
            destination
        )
    };
    
    command.arg(&user_path);
    
    debug!("Executing {}", format!("{:?}", command));

    let mut child = command.spawn().expect("Failed to execute ping");
    child.wait().expect("Failed to wait on child");
}

pub fn ssh(config: &MachineConfig, user: Option<&str>, tmux: bool) {
    let mut command = Command::new("ssh");
    
    if config.identity.is_some() {
        command.args(&["-i", config.identity.as_ref().unwrap()]);
    }
    
    if config.port.is_some() {
        command.args(&["-p", &(config.port.as_ref().unwrap().to_string())]);
    } else {
        command.args(&["-p", "22"]);
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
    
    let user_path = if user_name.is_some() {
        format!("{}@{}",
            user_name.unwrap(),
            config.ip.as_ref().unwrap()
        )
    } else {
        format!("{}",
            config.ip.as_ref().unwrap()
        )
    };
    
    command.arg(&user_path);
    
    if tmux {
        command.arg("tmux attach || tmux new");
    }
    
    debug!("Executing {}", format!("{:?}", command));
    
    let mut child = command.spawn().expect("Failed to execute ping");
    child.wait().expect("Failed to wait on child");
}