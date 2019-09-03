//! # Factomd HTTP RPC API
//!
//! This is the most crucial layer for backwards compatibility. Clients running on factomd must be
//! able to drop in this version without complication.
//!
pub fn start_rpc_server(_addr: &str, _port: u16) {}
