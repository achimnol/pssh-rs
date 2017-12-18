//! Shell

use std::env;

use clap::{Arg, SubCommand, App};

use config::load_configuration_file;

const VERSION: &str = "1.0.0";

pub fn init_shell() {
    let mut app = App::new("pssh")
        .version(VERSION)
        .author("Denis B. <bourge.denis@gmail.com>")
        .about("pssh")
        .arg(Arg::with_name("file")
            .long("file")
            .short("f")
            .value_name("FILENAME")
            .help("Use a custom file")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("verbose mode"))
            
        .subcommand(SubCommand::with_name("connect")
            .about("connect to a machine")
            .arg(Arg::with_name("machine")
                .value_name("MACHINE")
                .help("machine name")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("user")
                .value_name("USERNAME")
                .long("user")
                .short("u")
                .help("set username")
                .takes_value(true))
            .arg(Arg::with_name("tmux")
                .long("tmux")
                .short("t")
                .help("use tmux")))
        
        .subcommand(SubCommand::with_name("list")
            .about("list available machines"))
        
        .subcommand(SubCommand::with_name("push")
            .about("push file to a machine")
            .arg(Arg::with_name("machine")
                .value_name("MACHINE")
                .help("machine name")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("source")
                .value_name("FILE")
                .help("source filename")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("destination")
                .value_name("FILE")
                .help("destination filename")
                .required(true)
                .takes_value(true)))
        
        .subcommand(SubCommand::with_name("pull")
            .about("pull file from a machine")
            .arg(Arg::with_name("machine")
                .value_name("MACHINE")
                .help("machine name")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("source")
                .value_name("FILE")
                .help("source filename")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("destination")
                .value_name("FILE")
                .help("destination filename")
                .required(true)
                .takes_value(true)))
                
        .subcommand(SubCommand::with_name("ping")
            .about("ping a machine")
            .arg(Arg::with_name("machine")
                .value_name("MACHINE")
                .help("machine name")
                .required(true)
                .takes_value(true)))
        
        .subcommand(SubCommand::with_name("show")
            .about("show machine info")
            .arg(Arg::with_name("machine")
                .value_name("MACHINE")
                .help("machine name")
                .required(true)
                .takes_value(true)));
            
    let matches = app.get_matches_from_safe_borrow(&mut env::args_os());    
    match matches {
        Ok(result) => {            
            let config_file = result.value_of("file").and_then(|s| Some(String::from(s)));
            
            match result.subcommand() {
                ("list", Some(args)) => handle_list(config_file),
                ("show", Some(args)) => handle_show(config_file, args.value_of("machine").unwrap()),
                ("pull", Some(args)) => handle_pull(
                    config_file,
                    args.value_of("machine").unwrap(),
                    args.value_of("source").unwrap(),
                    args.value_of("destination").unwrap()
                ),
                ("push", Some(args)) => handle_push(
                    config_file,
                    args.value_of("machine").unwrap(),
                    args.value_of("source").unwrap(),
                    args.value_of("destination").unwrap()
                ),
                ("ping", Some(args)) => handle_ping(config_file, args.value_of("machine").unwrap()),
                ("connect", Some(args)) => handle_connect(
                    config_file,
                    args.value_of("machine").unwrap(),
                    args.value_of("user"),
                    args.is_present("tmux")
                ),
                _ => {
                    app.print_help().ok();
                }
            }
        },
        
        Err(error) => {
            eprintln!("{}", error.to_string());
        }
    }
}

fn handle_list(config_file: Option<String>) {
    load_configuration_file(config_file);
    
    println!("list machines !");
}

fn handle_show(config_file: Option<String>, machine: &str) {
    println!("showing machine {}", machine);
}

fn handle_pull(config_file: Option<String>, machine: &str, source: &str, destination: &str) {
    println!("pulling file {} from machine {} to {}", source, machine, destination);
}

fn handle_push(config_file: Option<String>, machine: &str, source: &str, destination: &str) {
    println!("pushing file {} to machine {} at {}", source, machine, destination);
}

fn handle_ping(config_file: Option<String>, machine: &str) {
    println!("pinging machine {}", machine);
}

fn handle_connect(config_file: Option<String>, machine: &str, user: Option<&str>, tmux: bool) {
    println!("connecting to machine {}", machine);
    println!("user: {:?}, tmux: {}", user, tmux);
}
