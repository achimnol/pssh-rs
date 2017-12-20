extern crate clap;
extern crate yaml_rust;

#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

#[macro_use]
extern crate maplit;

mod config;
mod shell;
mod wrapper;

pub use shell::init_shell;