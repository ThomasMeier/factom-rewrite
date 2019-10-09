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

use core::str::FromStr;
use factomd_configuration::{FactomConfig, Log, LogLevel, Role};
use futures::{future, sync::oneshot, Future};
use slog::{Drain, Logger, Duplicate};
use std::cell::RefCell;
use std::ops::Deref;
use std::sync::Mutex;
use std::fs::OpenOptions;
pub use substrate_cli::{error, informant, parse_and_execute, IntoExit, NoCustom, VersionInfo};
use substrate_service::ServiceFactory;
use tokio::runtime::Runtime;

mod chain_spec;
mod wrapper;

/// Build a Logger for the Factom Daemon
///
/// Drain is terminal
fn make_logger(log_config: &Log) -> Logger {
    let log_level_config = &log_config.log_level;
    let log_level = slog::Level::from_str(&log_level_config.to_string()).unwrap();

    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_config.log_path)
        .unwrap();

    let term_decorator = slog_term::TermDecorator::new().build();
    let file_decorator = slog_term::PlainDecorator::new(log_file);
    let term_drain = Mutex::new(slog_term::FullFormat::new(term_decorator).build())
        .filter_level(log_level)
        .fuse();
    let file_drain = Mutex::new(slog_term::FullFormat::new(file_decorator).build())
        .filter_level(log_level)
        .fuse();

    let root = Logger::root(Duplicate::new(term_drain, file_drain).fuse(), o!());

    slog::Logger::root(root, o!("version" => env!("CARGO_PKG_VERSION")))
}

/// Start the API server
fn start_rpc_server(log_option: Option<Logger>, config: &FactomConfig) {
    match log_option {
        Some(log) => {
            // Start HTTP RPC if enabled
            if !config.rpc.disable_rpc {
                info!(log, "HTTP RPC server enabled"; "addr" => &config.rpc.rpc_addr, "port" => &config.rpc.rpc_port);
                factomd_rpc::start_rpc_server(&config.rpc.rpc_addr, config.rpc.rpc_port);
            } else {
                info!(log, "HTTP RPC Server disabled");
            }
        }
        None => {
            if !config.rpc.disable_rpc {
                factomd_rpc::start_rpc_server(&config.rpc.rpc_addr, config.rpc.rpc_port);
            }
        }
    }
}

/// # Create Substrate-specific args
///
/// The issue is that susbtrate selects a chain spec to use
/// based on cli args provided by the user. We can't simply pass
/// FactomConfig over to substrate, they want strings typed by
/// the user.
///
/// So this is a workaround.
///
/// The function `substrate-cli::parse_and_execute` takes those args
/// down in our `run` fn. It is possible to pass our own structopt,
/// but this leads to collisions with how they've named some options.
///
/// That would be preferable. The biggest barrier for us to overcome is
/// the setting of rpc port, addr, and disabled. Because we will have
/// an rpc server for backward compatibility with existing Factom
/// clients. And substrate will try to launch its own rpc server,
/// which in production will be disabled.
///
/// (End users would then see options to set rpc port for
/// the OOTB Substrate and the rpc port for legacy rpc server. Not cool.)
///
/// `create_substrate_args` will take FactomConfig and turn it into
/// a vector for use as args by substrate-cli. Those settings include:
///
/// * Chain/Network to join
/// * Node key for the node to use when in validator role
/// * Set Validator mode when in validator role
fn create_substrate_args(factom_config: &FactomConfig) -> Vec<String> {
    let node_key;
    let mut args = Vec::new();
    args.push("_".to_string());

    if factom_config.server.role == Role::AUTHORITY {
        node_key = std::env::var(&factom_config.server.node_key_env)
            .unwrap_or_else(|_| panic!("Failed to find node key"));
        args.push("--validator".to_string());
        args.push("--node-key".to_string());
        args.push(node_key);
    }

    if !factom_config.server.base_path.is_empty() {
        args.push("--base-path".to_string());
        args.push(factom_config.server.base_path.to_string());
    }

    args.push("--bootnodes".to_string());
    args.push(factom_config.server.bootnodes.to_string());

    args.push("--port".to_string());
    args.push(factom_config.server.port.to_string());

    args.push("--chain=".to_string() + &factom_config.server.network);
    args
}

/// Start new node
pub fn run(factom_config: FactomConfig) -> Result<(), substrate_cli::error::Error> {
    // Build logger except if variant is SILENT.
    let log_option = match &factom_config.log.log_level {
        LogLevel::SILENT => None,
        _ => Some(make_logger(&factom_config.log)),
    };

    start_rpc_server(log_option, &factom_config);
    let runtime = Runtime::new().map_err(|e| format!("{:?}", e))?;
    let executor = runtime.executor();

    let version = VersionInfo {
        name: "Substrate Node",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "factomd",
        author: "tom",
        description: "factomd node",
        support_url: "https://github.com/ThomasMeier/factom-rewrite",
    };

    // Empty vector to pass to substrate factory as a placeholder.
    let substrate_args = create_substrate_args(&factom_config);

    parse_and_execute::<wrapper::Factory, NoCustom, NoCustom, _, _, _, _, _>(
        load_spec,
        &version,
        "factom-node",
        substrate_args,
        Exit,
        |exit, _custom_args, config| {
            match &factom_config.server.role {
                Role::LIGHT => run_until_exit(
                    runtime,
                    wrapper::Factory::new_light(config, executor)
                        .map_err(|e| format!("{:?}", e))?,
                    exit,
                ),
                _ => run_until_exit(
                    runtime,
                    wrapper::Factory::new_full(config, executor).map_err(|e| format!("{:?}", e))?,
                    exit,
                ),
            }
            .map_err(|e| format!("{:?}", e))
        },
    )
    .map_err(Into::into)
    .map(|_| ())
}

fn load_spec(id: &str) -> Result<Option<chain_spec::ChainSpec>, String> {
    Ok(match chain_spec::Alternative::from(id) {
        Some(spec) => Some(spec.load()?),
        None => None,
    })
}

fn run_until_exit<T, C, E>(mut runtime: Runtime, service: T, e: E) -> error::Result<()>
where
    T: Deref<Target = substrate_service::Service<C>>,
    C: substrate_service::Components,
    E: IntoExit,
{
    let (exit_send, exit) = exit_future::signal();

    let executor = runtime.executor();
    informant::start(&service, exit.clone(), executor.clone());

    let _ = runtime.block_on(e.into_exit());
    exit_send.fire();

    let _telemetry = service.telemetry();
    drop(service);
    Ok(())
}

pub struct Exit;
impl IntoExit for Exit {
    type Exit = future::MapErr<oneshot::Receiver<()>, fn(oneshot::Canceled) -> ()>;
    fn into_exit(self) -> Self::Exit {
        let (exit_send, exit) = oneshot::channel();

        let exit_send_cell = RefCell::new(Some(exit_send));
        ctrlc::set_handler(move || {
            if let Some(exit_send) = exit_send_cell
                .try_borrow_mut()
                .expect("signal handler not reentrant; qed")
                .take()
            {
                exit_send.send(()).expect("Error sending exit notification");
            }
        })
        .expect("Error setting Ctrl-C handler");

        exit.map_err(drop)
    }
}
