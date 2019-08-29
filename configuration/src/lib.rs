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
#[macro_use]
extern crate clap;
extern crate config;
extern crate serde;
extern crate structopt;

use clap::{App, _clap_count_exprs, arg_enum};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::io;
use structopt::clap::Shell;
use structopt::StructOpt;

// Log Levels Enum
arg_enum! {
    #[derive(Debug, Deserialize, PartialEq)]
    pub enum LogLevel {
        CRITICAL,
        ERROR,
        WARN,
        INFO,
        DEBUG,
        TRACE
    }
}

// Roles Enum
arg_enum! {
    #[derive(Debug, Deserialize, PartialEq)]
    pub enum Role {
        FULL,
        LIGHT,
        AUTHORITY
    }
}

/// Logging Settings
#[derive(StructOpt, Debug, Deserialize)]
pub struct Log {
    #[structopt(
        short = "l",
        long = "log-level",
        default_value = "DEBUG",
        raw(possible_values = "&LogLevel::variants()")
    )]
    pub log_level: LogLevel,
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

    /// Environment variable to source for your node_key
    #[structopt(
        long = "role",
        short = "r",
        default_value = "FULL",
        raw(possible_values = "&Role::variants()", case_insensitive = "false")
    )]
    pub role: Role,

    /// Environment variable to source for your node_key
    #[structopt(long = "node-key-env", short = "k", default_value = "")]
    pub node_key_env: String,
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
    pub server: Server,

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
    #[structopt(long = "completions", raw(possible_values = "&Shell::variants()"))]
    pub completions: Option<String>,
}

/// Factom Configuration has a specific override order
/// Higher precedence -> lower precedence
/// CLI Args -> Environment Vars -> Custom Config -> default_config.yml
impl FactomConfig {
    /// Generate a new FactomConfig from multiple sources
    ///
    /// config-rs is mutable, must have CLI gathered to start, a lot of mutation and side effects not avoidable
    pub fn new() -> Result<Self, ConfigError> {
        let yaml = load_yaml!("../cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        // Just to get a custom config path
        let cli_args = FactomConfig::from_args();

        // If a completions argument is provided, dump shell completion to stdout
        if let Some(completion) = cli_args.completions {
            FactomConfig::completions_to_stdout(&completion);
        }

        let file_args = FactomConfig::load_from_path(&cli_args.custom_config)?;
        let final_config = FactomConfig::check_cli(matches, file_args);

        Ok(final_config)
    }

    pub fn load_from_path(path: &str) -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        // Get defaults
        settings.merge(File::with_name("default_config.yml"))?;

        settings.set("custom_config", "")?;
        settings.set("completions", "")?;

        // If custom_config isn't an empty string, put the custom config
        // on top of the default
        if !path.is_empty() {
            settings.merge(File::with_name(path))?;
        }

        // Read any configs in environment, search any prefixed FACTOMD_
        settings.merge(Environment::with_prefix("factomd"))?;

        settings.try_into()
    }

    /// Check for supplied command line arguments and use them to overwrite config values from files.
    fn check_cli(matches: clap::ArgMatches, mut config: FactomConfig) -> FactomConfig {
        if matches.occurrences_of("network") > 0 {
            if let Some(value) = matches.value_of("network") {
                config.server.network = value.to_string();
            }
        }
        if matches.occurrences_of("role") > 0 {
            if let Some(value) = matches.value_of("role") {
                config.server.role = value.parse::<Role>().expect("Invalid role type!");
            }
        }
        if matches.occurrences_of("node_key_env") > 0 {
            if let Some(value) = matches.value_of("node_key_env") {
                config.server.node_key_env = value.to_string();
            }
        }
        if matches.occurrences_of("log_level") > 0 {
            if let Some(value) = matches.value_of("log_level") {
                config.log.log_level = value.parse::<LogLevel>().expect("Invalid log level!");
            }
        }
        if matches.occurrences_of("walletd_user") > 0 {
            if let Some(value) = matches.value_of("walletd_user") {
                config.walletd.walletd_user = value.to_string();
            }
        }
        if matches.occurrences_of("walletd_env_var") > 0 {
            if let Some(value) = matches.value_of("walletd_env_var") {
                config.walletd.walletd_env_var = value.to_string();
                println!("{:?}", config.walletd.walletd_env_var);
            }
        }
        if matches.occurrences_of("rpc_addr") > 0 {
            if let Some(value) = matches.value_of("rpc_addr") {
                config.rpc.rpc_addr = value.to_string();
            }
        }
        if matches.occurrences_of("rpc_port") > 0 {
            if let Some(value) = matches.value_of("rpc_port") {
                config.rpc.rpc_port = value.parse::<u16>().expect("Invalid port value!");
            }
        }
        if matches.occurrences_of("disable_rpc") > 0 {
            config.rpc.disable_rpc = true;
        }
        config
    }

    pub fn completions_to_stdout(shell: &str) {
        let selected: Shell = shell.parse().unwrap();
        FactomConfig::clap().gen_completions_to(
            env!("CARGO_PKG_NAME"),
            selected,
            &mut io::stdout(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::process::Command;
    // use std::ffi::OsString;
    // use assert_cmd::prelude::*;

    #[test]
    fn test_default() {
        let config = FactomConfig::new().unwrap();

        assert_eq!(config.rpc.rpc_addr, "127.0.0.1");
        assert_eq!(config.rpc.rpc_port, 8088);
        assert_eq!(config.server.network, "main");
        assert_eq!(config.log.log_level, LogLevel::DEBUG);
        assert_eq!(config.walletd.walletd_user, "");
        assert_eq!(config.walletd.walletd_env_var, "");
        assert_eq!(config.server.role, Role::FULL);
        assert_eq!(config.server.node_key_env, "");
    }

    #[test]
    fn test_nondefault_yaml() {
        let nondefault_config = FactomConfig::load_from_path("tests/nondefaults.yml").unwrap();
        assert_eq!(nondefault_config.rpc.rpc_addr, "192.0.0.1");
        assert_eq!(nondefault_config.rpc.rpc_port, 7777);
        assert_eq!(nondefault_config.server.network, "test");
        assert_eq!(nondefault_config.log.log_level, LogLevel::CRITICAL);
        assert_eq!(nondefault_config.walletd.walletd_user, "test");
        assert_eq!(nondefault_config.walletd.walletd_env_var, "TEST");
        assert_eq!(nondefault_config.server.role, Role::LIGHT);
        assert_eq!(nondefault_config.server.node_key_env, "FACTOMD_NODE_KEY");
    }

    #[test]
    // Ensure check_cli() function overwrites config file values.
    fn test_check_cli() {
        let file_args = FactomConfig::load_from_path("tests/nondefaults.yml").unwrap();
        let vec = vec![
            "factomd",
            "--role",
            "AUTHORITY",
            "--rpc-port",
            "8099",
            "--rpc-addr",
            "8.8.8.8",
            "--disable-rpc",
            "--network",
            "custom",
            "--node-key-env",
            "NODE_KEY_EXAMPLE",
            "--walletd-user",
            "USER123",
            "--walletd-env-var",
            "WALLETD_ENV",
            "--log-level",
            "WARN",
        ];

        let yaml = load_yaml!("../cli.yml");
        let app = App::from_yaml(yaml);

        let matches = App::get_matches_from(app, vec);

        let final_config = FactomConfig::check_cli(matches, file_args);

        assert_eq!(final_config.server.role, Role::AUTHORITY);
        assert_eq!(final_config.rpc.rpc_port, 8099);
        assert_eq!(final_config.rpc.rpc_addr, "8.8.8.8");
        assert_eq!(final_config.rpc.disable_rpc, true);
        assert_eq!(final_config.server.network, "custom");
        assert_eq!(final_config.server.node_key_env, "NODE_KEY_EXAMPLE");
        assert_eq!(final_config.walletd.walletd_user, "USER123");
        assert_eq!(final_config.walletd.walletd_env_var, "WALLETD_ENV");
        assert_eq!(final_config.log.log_level, LogLevel::WARN);
    }
}
