//! # Factom Service
//! 
//! Start a new node based on provided configuration.
//! 
//! This will start the HTTP RPC API if enabled. It will set up a 
//! logger for the program to use, and start a new Substrate service
//! and runtime.
//! 
#[macro_use]

extern crate slog;
extern crate slog_async;
extern crate slog_term;

use std::sync::Mutex;
use slog::Drain;
use slog::Logger;
use core::str::FromStr;
use factomd_configuration::Log;
use factomd_configuration::FactomConfig;
pub use substrate_cli::{VersionInfo, IntoExit, error};

/// Build a Logger for the Factom Daemon
/// 
/// Drain is terminal
fn make_logger(log_config: &Log) -> Logger {
    let log_level_config = &log_config.log_level;
    let log_level = slog::Level::from_str(&log_level_config.to_string()).unwrap();
    let decorator = slog_term::TermDecorator::new().build();
    let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).filter_level(log_level).fuse();
    slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")))
}

/// Start the API server
fn start_api(log: Logger, config: &FactomConfig) {
      // Start HTTP RPC if enabled
    if !config.rpc.disable_rpc {
        info!(log, "HTTP RPC server enabled"; "addr" => &config.rpc.rpc_addr, "port" => &config.rpc.rpc_port);
        factomd_rpc::start_rpc_server(&config.rpc.rpc_addr, config.rpc.rpc_port);
    } else {
        info!(log, "HTTP RPC Server disabled");
    }
}

/// Start new node
pub fn run(config: FactomConfig) -> Result<(), substrate_cli::error::Error> {
    let log = make_logger(&config.log);
    start_api(log, &config);
    Ok(())
}