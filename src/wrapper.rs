//! Shell wrappers

use std::process::Command;

use config::MachineConfig;

/// SSH Copy direction
pub enum ScpDirection {
    /// Push direction (host -> machine)
    Push,
    /// Pull direction (machine -> host)
    Pull
}

/// Ping a machine
///
/// # Arguments
///
/// * `ip` - Machine IP
///
pub fn ping(ip: &str) -> Command {
    let mut command = Command::new("ping");
    command.arg(ip);      
        
    debug!("Executing {}", format!("{:?}", command));
    command
}

/// Copy a file from machine to host
///
/// # Arguments
///
/// * `config` - Machine configuration
/// * `source` - Source path
/// * `destination` - Destination path
///
pub fn scp(config: &MachineConfig, source: &str, destination: &str, direction: ScpDirection) -> Command {
    let mut command = Command::new("scp");
    
    if config.identity.is_some() {
        command.args(&["-i", config.identity.as_ref().unwrap()]);
    }
    
    if config.port.is_some() {
        command.args(&["-P", &(config.port.as_ref().unwrap().to_string())]);
    } else {
        command.args(&["-P", "22"]);
    }

    let machine_path = match direction {
        ScpDirection::Push => destination,
        ScpDirection::Pull => source
    };
        
    let user_path = if config.user.is_some() {
        format!("{}@{}:{}",
            &config.user.as_ref().unwrap(),
            &config.ip.as_ref().unwrap(),
            machine_path
        )
    } else {
        format!("{}:{}",
            &config.ip.as_ref().unwrap(),
            machine_path
        )
    };
    
    match direction {
        ScpDirection::Push => {
            command.arg(&source);
            command.arg(&user_path);
        },
        ScpDirection::Pull => {
            command.arg(&user_path);
            command.arg(&destination);
        }
    }
    
    debug!("Executing {}", format!("{:?}", command));
    command
}

/// Execute an SSH connection
///
/// # Arguments
///
/// * `config` - Machine configuration
/// * `user` - Username
/// * `tmux` - Use `tmux`
///
pub fn ssh(config: &MachineConfig, user: Option<&str>, tmux: bool) -> Command {
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
    command
}

/// Execute a command
///
/// # Arguments
///
/// * `command` - Command to execute
/// * `error_message` - Error message
/// 
pub fn execute(mut command: Command, error_message: &str) {
    let mut child = command.spawn().expect(error_message);
    child.wait().expect("Failed to wait on child");
}

#[cfg(test)]
mod test {
    use super::*;

    fn format_command(command: &Command) -> String {
        let debug_cmd = format!("{:?}", command);
        debug_cmd.split(" ").map(|s| {
            let length = s.len();
            s.chars().skip(1).take(length - 2).collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
    }

    #[test]
    fn test_commands() {
        let config = MachineConfig {
            ip: Some("localhost".to_string()),
            .. Default::default()
        };

        let command = scp(&config, "/toto", "./tutu", ScpDirection::Push);        
        assert_eq!(format_command(&command), "scp -P 22 /toto localhost:./tutu");

        let command = scp(&config, "/toto", "./tutu", ScpDirection::Pull);        
        assert_eq!(format_command(&command), "scp -P 22 localhost:/toto ./tutu");

        let command = ssh(&config, None, false);
        assert_eq!(format_command(&command), "ssh -p 22 localhost");

        let command = ssh(&config, Some("toto"), false);
        assert_eq!(format_command(&command), "ssh -p 22 toto@localhost");
    }
}
