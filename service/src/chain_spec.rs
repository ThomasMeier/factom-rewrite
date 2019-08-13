//! # Factom Chain Spec
//! 
//! Define the networks that are available to factomd.
//! 
use service;

#[derive(Clone, Debug)]
pub enum ChainSpec {
	Development,
	LocalTestnet,
}

impl Default for ChainSpec {
	fn default() -> Self {
		ChainSpec::LocalTestnet
	}
}

impl ChainSpec {
	pub(crate) fn load(self) -> Result<service::ChainSpec, String> {
		Ok(match self {
			ChainSpec::Development => service::chain_spec::development_config(),
			ChainSpec::LocalTestnet => service::chain_spec::local_testnet_config(),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(ChainSpec::Development),
			"local" => Some(ChainSpec::LocalTestnet),
			_ => None,
		}
	}
}