//! # Factom Chain Spec
//! 
//! Define the chain behavior for factomd. Set network alternatives. Set behaviors based on network alternatives.
//! Set telemetry URL. Set public accounts for staging/local QA and developers to use for testing. Set authority 
//! nodes for testing purposes and initial launch. Set network behaviors like block time.
//! 
//! 
use primitives::{ed25519, sr25519, Pair};
use substrate_service;
use ed25519::Public as AuthorityId;

/// Alternatives available for network
#[derive(Clone, Debug)]
pub enum ChainSpec {
	// A single authority node "Alice" for public networking availability
	Development,
	// Five public test accounts are available for localhost nodes, 2 authorities
	LocalTestnet,
}

/// Get AuthorityId from str
///
/// This will generate a ed25519 pair from a given string
/// in Substrate it is common to use the first names of
/// the mock users, e.g., "Alice"
fn authority_key(s: &str) -> AuthorityId {
	ed25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valid; qed")
		.public()
}

/// Get AuthorityId from str
/// 
/// This will generate a sr25519 pair from a given string
/// in Substrate it is common to use the first names of
/// the mock users, e.g., "Alice"
fn account_key(s: &str) -> AccountId {
	sr25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valid; qed")
		.public()
}

/// Implement an alternative
/// 
/// This is selected from the factomd-configuration
impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	/// 
	/// Alice has the root key for both networks.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| make_genesis(vec![
					authority_key("Alice")
				], vec![
					account_key("Alice")
				],
					account_key("Alice")
				),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Local Testnet",
				"local_testnet",
				|| make_genesis(vec![
					authority_key("Alice"),
					authority_key("Bob"),
				], vec![
					account_key("Alice"),
					account_key("Bob"),
					account_key("Charlie"),
					account_key("Dave"),
					account_key("Eve"),
					account_key("Ferdie"),
				],
					account_key("Alice"),
				),
				vec![],
				None,
				None,
				None,
				None
			),
		})
	}
	/// Get spec based on str provided from factomd-configuration
	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"" | "local" => Some(Alternative::LocalTestnet),
			_ => None,
		}
	}
}

/// Make hardcoded genesis
/// 
/// This is also possible via json, however the wasm blob will get thrown in. This 
/// should help keep things compact by simply hardcoding the genesis.
fn make_genesis(initial_authorities: Vec<AuthorityId>, endowed_accounts: Vec<AccountId>, root_key: AccountId) -> GenesisConfig {
	GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/factomd_runtime_wasm.compact.wasm").to_vec(),
			authorities: initial_authorities.clone(),
		}),
		system: None,
		timestamp: Some(TimestampConfig {
			minimum_period: 5,
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
			existential_deposit: 500,
			transfer_fee: 0,
			creation_fee: 0,
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
	}
}

