//! pssh - Simple tool to manage SSH connexions

#![warn(missing_docs)]

extern crate clap;
extern crate yaml_rust;

#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate maplit;

pub mod config;
pub mod shell;
pub mod wrapper;

pub use shell::init_shell;
