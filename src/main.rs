//! # The Factom Daemon
//! 
//! The Factom® Protocol is a blockchain trusted by the U.S Department of Homeland Security, the 
//! Bill and Melinda Gates Foundation and other leading enterprise. The technology provides high 
//! throughput and easy integration into legacy systems, without the need to handle 
//! cryptocurrency. The protocol allows fixed costs to accurately budget projects, whilst securing 
//! unlimited data via simple API integrations.
//! 
//! This software operates a node in the Factom network. See the README and the doc folder for 
//! more information on using this software.
//! 
#![warn(missing_docs)]
#![warn(unused_extern_crates)]

fn run() -> factomd_service::error::Result<()> {
     // Get config
    let config = factomd_configuration::FactomConfig::new().unwrap();

    // Start factoming
    factomd_service::run(config)
}

error_chain::quick_main!(run);