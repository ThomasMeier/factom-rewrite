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
use futures::{future, Future, sync::oneshot};
use tokio::runtime::Runtime;
use std::cell::RefCell;
use slog::Drain;
use slog::Logger;
use core::str::FromStr;
use std::ops::Deref;
use factomd_configuration::{Log, FactomConfig, Role};
use substrate_service::{ServiceFactory};
pub use substrate_cli::{VersionInfo, NoCustom, parse_and_execute, informant, IntoExit, error};

mod wrapper;
mod chain_spec;

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
fn start_rpc_server(log: Logger, config: &FactomConfig) {
      // Start HTTP RPC if enabled
    if !config.rpc.disable_rpc {
        info!(log, "HTTP RPC server enabled"; "addr" => &config.rpc.rpc_addr, "port" => &config.rpc.rpc_port);
        factomd_rpc::start_rpc_server(&config.rpc.rpc_addr, config.rpc.rpc_port);
    } else {
        info!(log, "HTTP RPC Server disabled");
    }
}

/// Start new node
pub fn run(factom_config: FactomConfig) -> Result<(), substrate_cli::error::Error> {
    let log = make_logger(&factom_config.log);
    start_rpc_server(log, &factom_config);
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

    parse_and_execute::<wrapper::Factory, NoCustom, NoCustom, _, _, _, _, _>(
        load_spec, &version, "factom-node", ::std::env::args(), Exit,
        |exit, _custom_args, config| {
        match &factom_config.server.role {
            Role::LIGHT => run_until_exit(
                runtime,
                wrapper::Factory::new_light(config, executor).map_err(|e| format!("{:?}", e))?,
                exit),
            _ => run_until_exit(
                runtime,
                wrapper::Factory::new_full(config, executor).map_err(|e| format!("{:?}", e))?,
                exit),
        }.map_err(|e| format!("{:?}", e))
        }
    ).map_err(Into::into).map(|_| ())
}

fn load_spec(id: &str) -> Result<Option<chain_spec::ChainSpec>, String> {
	Ok(match chain_spec::Alternative::from(id) {
		Some(spec) => Some(spec.load()?),
		None => None,
	})
}

fn run_until_exit<T, C, E>(
	mut runtime: Runtime,
	service: T,
	e: E,
) -> error::Result<()>
	where
		T: Deref<Target=substrate_service::Service<C>>,
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
			if let Some(exit_send) = exit_send_cell.try_borrow_mut().expect("signal handler not reentrant; qed").take() {
				exit_send.send(()).expect("Error sending exit notification");
			}
		}).expect("Error setting Ctrl-C handler");

		exit.map_err(drop)
	}
}
