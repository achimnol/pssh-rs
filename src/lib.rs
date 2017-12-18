extern crate clap;
extern crate yaml_rust;

mod config;
mod shell;
mod wrapper;

pub use shell::init_shell;