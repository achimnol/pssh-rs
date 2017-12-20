extern crate clap;
extern crate yaml_rust;

#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

mod config;
mod shell;
mod wrapper;

pub use shell::init_shell;