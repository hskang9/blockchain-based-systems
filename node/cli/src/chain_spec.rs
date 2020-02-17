// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use babe_primitives::AuthorityId as BabeId;
use chain_spec::ChainSpecExtension;
use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use im_online::sr25519::AuthorityId as ImOnlineId;
use node_runtime::constants::{currency::*, time::*};
use node_runtime::Block;
use node_runtime::{
	AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, ContractsConfig, CouncilConfig,
	DemocracyConfig, ElectionsConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig,
	SessionKeys, StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	WASM_BINARY,
};
use primitives::{crypto::UncheckedInto, Pair, Public};
use serde::{Deserialize, Serialize};
use sr_primitives::Perbill;
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;

pub use node_primitives::{AccountId, Balance};
pub use node_runtime::GenesisConfig;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: client::ForkBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig, Extensions>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
}

fn session_keys(grandpa: GrandpaId, babe: BabeId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys {
		grandpa,
		babe,
		im_online,
	}
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId)> = vec![
		(
			// Vault-1
			// 5Cu7LRpXinsD5Yac4JhVuwjZXWb9pYLaRrNFCqgEerpodShs
			hex!["24ead0c40800ad4fc9c875c0d446a877a9c17ee6c6875e9e03cde0043cadf804"]
				.unchecked_into(),
			// 5CG3g3TarEwK7GSMLTo21wYZpA8GnvKuFKF4YSyGRZbFAEPq
			hex!["08a668a39791334f267d3914787d4a81a8a34d6d07be4a8fb8c4e329901eb775"]
				.unchecked_into(),
			// 5ET7oNr2sLJRN5uuk3ehiDiSpcZAnBYkcATQDsE9QXkzU7h4
			hex!["6990774a5b728a1bda391c9f2d09583e065900095e1da3d434e051471f5c9c69"]
				.unchecked_into(),
			// 5CCkDf36E14k4FjW4eUDeg7Xz61E6bneZtpHU2MdoQrWf181
			hex!["0621e9f97d36e3df26e35deb54bcfce12e9335ceddb5dc3783f4a73a47d2614e"]
				.unchecked_into(),
			// 5CCkDf36E14k4FjW4eUDeg7Xz61E6bneZtpHU2MdoQrWf181
			hex!["0621e9f97d36e3df26e35deb54bcfce12e9335ceddb5dc3783f4a73a47d2614e"]
				.unchecked_into(),
		),
		(
			// Vault-2
			// 5DUe8Q7Jo67EM4Jwb3Tb52HZQSsqZPsWtUKmefKwSeWEWGBa
			hex!["3e7d9229c22093c3e386450035e2dc6ba075c229dcf4e6cb43398ecdf0e0c674"]
				.unchecked_into(),
			// 5Ek9JwBKGYA175DgpDxuiYgP3x2SXHqLKXGRGftHpchvmGVn
			hex!["768cb83b73f97894beab119271f0bcc7a12ba18d8a16554e8e99d418fe979623"]
				.unchecked_into(),
			// 5DKjEbz3YJqJ3cxck1M3Zm2U5AxmjTNzxDL5hHGgJGZ5AjxR
			hex!["37b18dce76b2446a126027c484d26fb555a3b092bbf53d6fa2c6ab8498ed7efa"]
				.unchecked_into(),
			// 5EsmYdSTuZtxVKDcoDtdJxxBSfwusT2NbfvGYWNCcqnEiTUo
			hex!["7c5d6b49c9d1f856411434596c3414eec8ce776aa8873b5579ca5e84ae1d652f"]
				.unchecked_into(),
			// 5EsmYdSTuZtxVKDcoDtdJxxBSfwusT2NbfvGYWNCcqnEiTUo
			hex!["7c5d6b49c9d1f856411434596c3414eec8ce776aa8873b5579ca5e84ae1d652f"]
				.unchecked_into(),
		),
		(
			// Vault-3
			// 5CFF3TN8U1g1bbpfs9vSjSsSDfomJSpTudkQH3U9A2HafAfr
			hex!["08096f763a538bf103384560c2f23f57c40b4aa047423e4acffdd6c2eae2fa26"]
				.unchecked_into(),
			// 5GnaJWzA7oRUJJGzR74bQbJfvezweEDzqeeVvTTH48yTQ1Mk
			hex!["d0dfcf87bae8ebd7f468cc89539f434b363e1bc79f7c1d6e9a7aa5d58b190631"]
				.unchecked_into(),
			// 5DLRWgwGk3pVbU7vHCfCafnxSUhx8Xq9Rg9QVH5MpoHL43rA
			hex!["383923b7e49725ef0f4e20743a449bbee67e06841f06130a9b8121109298a25b"]
				.unchecked_into(),
			// 5ECD4WsZi9QcGCGskM6D7g4BBiJrrDqLYNfs3wguda8z8Tgk
			hex!["5e317ebaf1c6fe79fd73478fe72d134a84ecaac34af3e2679e8548491af12002"]
				.unchecked_into(),
			// 5ECD4WsZi9QcGCGskM6D7g4BBiJrrDqLYNfs3wguda8z8Tgk
			hex!["5e317ebaf1c6fe79fd73478fe72d134a84ecaac34af3e2679e8548491af12002"]
				.unchecked_into(),
		),
		(
			// Vault-4
			// 5EhSR92ThPnqgwZghqRM9KwrwrosLFJMTHVwAUymVpd6wXX7
			hex!["747c921561833d9f83ded907d34a8429c740a17ca18f4fc6e587e857d378c81f"]
				.unchecked_into(),
			// 5He31oCRGSWWebKPwBWcP36qRLLEujhTaWVGAGaJjFuz6Rdu
			hex!["f698c15020c2cbab8733fe9700ad214c56ecea7fa2aba306bf0d9a9ec756ae60"]
				.unchecked_into(),
			// 5EcNP6gcJye9BubvTXRusz3cvNqLPL9XWe8jvVihZjyGMCmd
			hex!["709ec28eee5e2e9135aba96edc96665f2036eececa584ad255384bcebf4f97fd"]
				.unchecked_into(),
			// 5FLjsB3ZdSFyArivuWxB8bdDvf3jjn8V4PPp1ZtdB9S2buqM
			hex!["90ef63854ebca322601636e31175cc72407789a190ccc8ec4a916182dcb9472a"]
				.unchecked_into(),
			// 5FLjsB3ZdSFyArivuWxB8bdDvf3jjn8V4PPp1ZtdB9S2buqM
			hex!["90ef63854ebca322601636e31175cc72407789a190ccc8ec4a916182dcb9472a"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5H4KssC4L1YH4ASYj73ZJHLonYa3xFzumBbVyhGGvexd87RF
		"dce32d9bf7f034ce180e5999a3bd71372883e351ccae83c6edf96f1bc86d0e5d"
	]
	.unchecked_into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, root_key, Some(endowed_accounts), false)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Vault Staging Testnet",
		"staging_testnet",
		staging_testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(
			STAGING_TELEMETRY_URL.to_string(),
			0,
		)])),
		None,
		None,
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId) {
	(
		get_from_seed::<AccountId>(&format!("{}//stash", seed)),
		get_from_seed::<AccountId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	enable_println: bool,
) -> GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_from_seed::<AccountId>("Alice"),
			get_from_seed::<AccountId>("Bob"),
			get_from_seed::<AccountId>("Charlie"),
			get_from_seed::<AccountId>("Dave"),
			get_from_seed::<AccountId>("Eve"),
			get_from_seed::<AccountId>("Ferdie"),
			get_from_seed::<AccountId>("Alice//stash"),
			get_from_seed::<AccountId>("Bob//stash"),
			get_from_seed::<AccountId>("Charlie//stash"),
			get_from_seed::<AccountId>("Dave//stash"),
			get_from_seed::<AccountId>("Eve//stash"),
			get_from_seed::<AccountId>("Ferdie//stash"),
		]
	});

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = 100 * DOLLARS;

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			vesting: vec![],
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts
				.iter()
				.cloned()
				.chain(initial_authorities.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone()),
					)
				})
				.collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		elections: Some(ElectionsConfig {
			members: vec![],
			presentation_duration: 1 * DAYS,
			term_duration: 28 * DAYS,
			desired_seats: 0,
		}),
		contracts: Some(ContractsConfig {
			current_schedule: contracts::Schedule {
				enable_println, // this should only be enabled on development chains
				..Default::default()
			},
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig { key: root_key }),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		im_online: Some(ImOnlineConfig { keys: vec![] }),
		authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		membership_Instance1: Some(Default::default()),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![get_authority_keys_from_seed("Alice")],
		get_from_seed::<AccountId>("Alice"),
		None,
		true,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_from_seed::<AccountId>("Alice"),
		None,
		false,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local testnet",
		"local_testnet",
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::new_full;
	use service_test;
	use substrate_service::Roles;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![get_authority_keys_from_seed("Alice")],
			get_from_seed::<AccountId>("Alice"),
			None,
			false,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		service_test::connectivity(
			integration_test_config_with_two_authorities(),
			|config| new_full(config),
			|mut config| {
				// light nodes are unsupported
				config.roles = Roles::FULL;
				new_full(config)
			},
			true,
		);
	}
}
