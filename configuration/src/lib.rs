//! # Factomd Configuration
//! 
//! At present we are using structopt and clap to build up a layered configuration.
//! See the docs for [config](https://docs.rs/config/0.9.3/config/). Config generates
//! bash completion scripts and layers different vectors for configuration (env vars, files, etc).
//! 
//! Structopt takes a datastructure and turns it into the `--help` command and then provides all
//! cli parsing we need.
//! 
//! These will provide factomd with all the configurations provided by the user, for exmaple: using env var for 
//! walletd password, using file for node rpc settings, using cli for overrides.
//! 
extern crate config;
extern crate clap;
extern crate structopt;
extern crate serde;

use std::io;
use structopt::clap::Shell;
use clap::{arg_enum, _clap_count_exprs};
use structopt::StructOpt;
use serde::{Deserialize};
use config::{ConfigError, Config, File, Environment,};

// Log Levels Enum
arg_enum! {
    #[derive(Debug, Deserialize, PartialEq)]
    pub enum LogLevel {
        OFF,
        CRITICAL,
        ERROR,
        WARN,
        INFO,
        DEBUG,
        TRACE
    }
}

/// Logging Settings
#[derive(StructOpt, Debug, Deserialize)]
pub struct Log {
    #[structopt(
        short = "l", 
        long = "log-level", 
        default_value = "DEBUG", 
        raw(possible_values = "&LogLevel::variants()", case_insensitive = "false"))]
    pub log_level: LogLevel
}

/// Factom RPC API settings
#[derive(StructOpt, Debug, Deserialize)]
pub struct Rpc {
    /// Disable RPC server
    #[structopt(short = "d", long = "disable-rpc")]
    pub disable_rpc: bool,

     /// HTTP-RPC listening interface
    #[structopt(long = "rpc-addr", default_value = "127.0.0.1")]
    pub rpc_addr: String,

    /// HTTP-RPC listening port
    #[structopt(long = "rpc-port", default_value = "8088")]
    pub rpc_port: u16,
}

/// Walletd Settings
/// Takes a env var for password, may use a password entry input in the future
#[derive(StructOpt, Debug, Deserialize)]
pub struct Walletd {
        /// Set walletd user for authentication
    #[structopt(long = "walletd-user", default_value = "")]
    pub walletd_user: String,

    /// Set env variable to get walletd password
    #[structopt(long = "walletd-env-var", default_value = "")]
    pub walletd_env_var: String,
}

/// Factom Server Settings
#[derive(StructOpt, Debug, Deserialize)]
pub struct Server {
    /// Set network to join
    #[structopt(short = "n", long = "network", default_value = "main")]
    pub network: String,
}


/// FactomConfig used for setting up your Factom node
#[derive(StructOpt, Debug, Deserialize)]
#[structopt(name = "Factom")]
pub struct FactomConfig {
    /// Custom configuration file location
    #[structopt(short = "c", long = "config", default_value = "")]
    pub custom_config: String,

    #[allow(missing_docs)]
	#[structopt(flatten)]
    pub server:  Server,

    #[allow(missing_docs)]
	#[structopt(flatten)]
    pub log: Log,

    #[allow(missing_docs)]
	#[structopt(flatten)]
    pub rpc: Rpc,

    #[allow(missing_docs)]
	#[structopt(flatten)]
    pub walletd: Walletd,

    /// Generate completions
    #[structopt (long = "completions", default_value = "", raw(possible_values = "&Shell::variants()", case_insensitive = "false"))]
    pub completions: String
}

/// Factom Configuration has a specific override order
/// Higher precedence -> lower precedence
/// CLI Args -> Environment Vars -> Custom Config -> default_config.yml
impl FactomConfig {
  /// Generate a new FactomConfig from multiple sources
  /// 
  /// config-rs is mutable, must have CLI gathered to start, a lot of mutation and side effects not avoidable
  pub fn new() -> Result<Self, ConfigError> {
    // Just to get a custom config path
    let cli_args = FactomConfig::from_args();

    // Dump shell completion to stdout
    if !cli_args.completions.is_empty() {
        FactomConfig::completions_to_stdout(&cli_args.completions);
    }

    // TODO Layer cli_args on top of here see github issue #1
    FactomConfig::load_from_path(&cli_args.custom_config)
  }

  pub fn load_from_path(path: &str) -> Result<Self, ConfigError> {
    let mut settings = Config::new();
      // Get defaults
    settings.merge(File::with_name("default_config.yml"))?;

    settings.set("custom_config", "")?;

    // If custom_config isn't an empty string, put the custom config
    // on top of the default
    if !path.is_empty() {
      settings.merge(File::with_name(path))?;
    }

    // Read any configs in environment, search any prefixed FACTOMD_
    settings.merge(Environment::with_prefix("factomd"))?;

    settings.try_into()
  }

  pub fn completions_to_stdout(shell: &str) {
      let selected: Shell = shell.parse().unwrap();
      FactomConfig::clap().gen_completions_to(env!("CARGO_PKG_NAME"), selected, &mut io::stdout());
  }
}


// These tests can be moved to test dir if needed
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nondefault_yaml() {
        let nondefault_config = FactomConfig::load_from_path("tests/nondefaults.yml").unwrap();
        assert_eq!(nondefault_config.rpc.rpc_addr, "192.0.0.1");
        assert_eq!(nondefault_config.rpc.rpc_port, 7777);
        assert_eq!(nondefault_config.server.network, "test");
        assert_eq!(nondefault_config.log.log_level, LogLevel::CRITICAL);
        assert_eq!(nondefault_config.walletd.walletd_user, "test");
        assert_eq!(nondefault_config.walletd.walletd_env_var, "TEST");

    }
}