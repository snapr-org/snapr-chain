use sp_core::{Pair, Public, sr25519, H160, Bytes};
use snapr_runtime::{
	AccountId, CurrencyId,
	BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig, SudoConfig, SystemConfig,
	IndicesConfig, EvmConfig, StakingConfig, SessionConfig, AuthorityDiscoveryConfig,
	WASM_BINARY,
	TokenSymbol, TokensConfig, SNAPR,
	StakerStatus,
	ImOnlineId, AuthorityDiscoveryId,
	MaxNativeTokenExistentialDeposit,
	get_all_module_accounts,
	opaque::SessionKeys,
};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount};
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;

use sp_std::{collections::btree_map::BTreeMap, str::FromStr};
use sc_chain_spec::ChainSpecExtension;

use serde::{Deserialize, Serialize};

use hex_literal::hex;
use sp_core::{crypto::UncheckedInto, bytes::from_hex};

use snapr_primitives::{AccountPublic, Balance, Nonce};

// The URL for the telemetry server.
const TELEMETRY_URL: &str = "wss://telemetry.snapr.systems/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<snapr_primitives::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<snapr_primitives::Block>,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn get_session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
	) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, authority_discovery }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an authority keys.
pub fn get_authority_keys_from_seed(seed: &str)
	-> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				get_authority_keys_from_seed("Trillian"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Trillian"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Trillian"),
				get_account_id_from_seed::<sr25519::Public>("Ford"),
				get_account_id_from_seed::<sr25519::Public>("Trillian//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ford//stash"),
			],
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(snapr_properties()),
		// Extensions
		Default::default(),
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// initial PoA authorities
			vec![
				get_authority_keys_from_seed("Trillian"),
				get_authority_keys_from_seed("Ford"),
				get_authority_keys_from_seed("Arthur"),
			],
			// sudo account
			get_account_id_from_seed::<sr25519::Public>("Trillian"),
			// pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Trillian"),
				get_account_id_from_seed::<sr25519::Public>("Ford"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Trillian//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ford//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
		),
		// bootnodes
		vec![],
		// telemetry
		// TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		None,
		// Protocol ID
		Some("snapr_local_testnet"),
		// Properties
		Some(snapr_properties()),
		// Extensions
		Default::default(),
	))
}

pub fn public_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		"snapr testnet", // name
		"snapr_testnet", // id
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			// initial authorities keys:
			vec![
				(
					// stash
					hex!["b2902b07056f7365bc22bf7e69c4e4fdba03e6af9c73ca6eb1703ccbc0248857"].into(),
					// controller
					hex!["cc2ea454844cc1a2e821198d9e0ce1de1aee7d014af5dd3404fc8199df89f821"].into(),
					// grandpa
					hex!["607712f6581e191b69046427a7e33c4713e96b4ae4654e2467c74279dc20beb2"].unchecked_into(),
					// babe
					hex!["ba630d2df03743a6441ab9221a25fc00a62e6f3b56c6920634eebb72a15fc90f"].unchecked_into(),
					// im-online
					hex!["72c0d10c9cd6e44ccf5e7acf0bb1b7c4d6987dda55a36343f3d45b54ad8bfe32"].unchecked_into(),
					// authority-discovery
					hex!["f287831caa53bc1dce6f0d676ab43d248921a4c34535be8f7d7d153eda29dc3f"].unchecked_into(),
				),
				(
					// stash
					hex!["06ee8fc0e34e40f6f2c98328d70874c6dd7d7989159634c8c87301efbcbe4470"].into(),
					// controller
					hex!["9cf9f939c16ef458e677472ff113af53e7fb9139244fcfa6fccb765aa8831019"].into(),
					// grandpa
					hex!["db6d2cb33abebdc024a14ef7bfbc68823660be8d1acac66770e406e484de3184"].unchecked_into(),
					// babe
					hex!["d09f879b3273d2cedab83fa741cdac328679c98914dc8dc07e359e19f0379844"].unchecked_into(),
					// im-online
					hex!["8c38deff9ab24a8c49e2b4fbdc963af7cbf06f99d6aabfaa6e50bfe6ae0d071d"].unchecked_into(),
					// authority-discovery
					hex!["dcc1644697e98d4171a29074a4bfaeb49b39b6ea91a8ec5e049d23ea3c4a4134"].unchecked_into(),
				),
				(
					// stash
					hex!["48267bffea5e524f1c0e06cce77f0ef920be7ed9a7dd47705e181edad64f532a"].into(),
					// controller
					hex!["38594d7640612c49337f3a0bc7b39232b86f9c9c4fedec3f8b00e45d3f073a2d"].into(),
					// grandpa
					hex!["c8996b17688cab9bcda8dafb4dde9bab4d9b1dc81c71419fca46fedcba74a14e"].unchecked_into(),
					// babe
					hex!["568c17ce5ef308bd9544e7b16f34089a2c2329193f31577a830ffe8a023a6874"].unchecked_into(),
					// im-online
					hex!["66db4135f59db92ce98cdd6c29befaf21a93f1a9059adc2326c7d371a214f97d"].unchecked_into(),
					// authority-discovery
					hex!["00858734321b53f0987a45906cbb91fe7ce1588fce03758c7c07f09022372c30"].unchecked_into(),
				),
			],
			// sudo
			hex!["0c994e7589709a85128a6695254af16227f7873816ae0269aa705861c315ba1e"].into(),
			// endowed accounts
			vec![
				hex!["0c994e7589709a85128a6695254af16227f7873816ae0269aa705861c315ba1e"].into(),
				hex!["9e42365c1a43fe7bd886118f49a2247aabda7079c3e4c5288f41afadd7bb1963"].into(),
				hex!["6c1371ce4b06b8d191d6f552d716c00da31aca08a291ccbdeaf0f7aeae51201b"].into(),
			],
		),
		// Bootnodes
		vec![
			"/dns/bootnode.testnet.snapr.systems/tcp/30334/p2p/12D3KooWKmFtS7BFtkkKWrP5ZcCpPFokmST2JFXFSsVBNeW5SXWg".parse().unwrap(),
		],
		// telemetry
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		// protocol ID
		Some("snapr_testnet"),
		// properties
		Some(snapr_properties()),
		// extensions
		Default::default(),
	))
}

pub fn mainnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		"snapr mainnet", // name
		"snapr_mainnet", // id
		ChainType::Live,
		move || mainnet_genesis(
			wasm_binary,

			// initial authority keys:
			vec![
				(
					// trillian stash
					hex!["c870ed11d1d98d8343375a08925f53ff8f9ce437623f89adac7f91f6bd1d6b35"].into(),
					// trillian controller
					hex!["1e062e1cf4e8b43f4b1e4d6b57cdcb562ee90789eb0f20d0d7b128b4c538757d"].into(),
					// trillian grandpa
					hex!["580224742778d565c81997d7138249774451c20ebd409e6e0101acac243348e1"].unchecked_into(),
					// trillian babe
					hex!["a81a913d800e00b6a3195f8a46b1e31295594458c70f7394354f1998acd52054"].unchecked_into(),
					// trillian im-online
					hex!["a8d34967c54e60a44c08ca55bb900d7ddedbdf6ac8d7e1d38e64fc65e432d21b"].unchecked_into(),
					// trillian authority-discovery
					hex!["dad7af38a21987496b34cd8a2d27fb75e99a6f3a1297e8c706b2d0019c245448"].unchecked_into(),
				),
				(
					// ford stash
					hex!["420d9c9edfae0de458861800ae68a73c9a4e69739dc553baa0db0f988bf57d0f"].into(),
					// ford controller
					hex!["f2b7edc424c1f6e7b878e7f3b4ad8b521c352576076ba83b3fec226597fffb46"].into(),
					// ford grandpa
					hex!["e7079e181fe7d4a1ffd40fd5595a8ca1ed31c94d203aba6514e1dde63951ea95"].unchecked_into(),
					// ford babe
					hex!["7ccbccddb2bc248b8f38d45a4c78d9b4af6a92171572507056864f10e7719127"].unchecked_into(),
					// ford im-online
					hex!["82ad236ecb5661038077f8fbabe54a63945e344705aa75a86f494777f1722121"].unchecked_into(),
					// ford authority-discovery
					hex!["7a30ce6370fd53297f9ff3a4fe8de374ccfc2e18f7d7672182de6eebdab45772"].unchecked_into(),
				),
				(
					// arthur stash
					hex!["a279d057de39911140fe8b930e000adc7bb6a89deabe9f03f4d0b589918e627a"].into(),
					// arthur controller
					hex!["a2ebbcd164e4545759db70fde6f40212cedb5afa05a8fd425a11883219eb6d3b"].into(),
					// arthur grandpa
					hex!["6df0b76c503bebbc94484c2041289163ea8ec6e1b472d19e496f1a1dfe1cfc2f"].unchecked_into(),
					// arthur babe
					hex!["2a38806dda6c67a2932a8af2bc7963f2881b100fd89794760c956e3a1387da04"].unchecked_into(),
					// arthur im-online
					hex!["c88388964bca32b4857cfac1a2b1be3b7c70025f9cd762ced947df33da546c62"].unchecked_into(),
					// arthur authority-discovery
					hex!["3c0f2bfd025fa4fc59a46113003ad1be991af55329de98fd43623d7fd57ef41a"].unchecked_into(),
				),
			],

			// sudo
			hex!["461998dda5ef0400a5f76eff16b83aa85a740d63b5f529a5f86de60441f4a110"].into(),

			// endowed accounts
			vec![
				// liquidity bridge reserves
				(hex!["1e8811e1d626f700bc77000a616bd6023921d0200a4f6d0365b1bf866ba57704"].into(), 2_000_000_000 as u128),

				// core nominators
				(hex!["f0e801be534460a020079b48879eeaa83f54292232160fe1f9235b73997d6304"].into(), 1_000_000_000 as u128),

				// contributor pool
				(hex!["8c536431b197223f298198b29a319a6913db6594c45df6ffe0891d2b1f95475b"].into(), 500_000_000 as u128),

				// sudo
				(hex!["461998dda5ef0400a5f76eff16b83aa85a740d63b5f529a5f86de60441f4a110"].into(), 100_000_000 as u128),

				// faucet stash (see: https://en.wikipedia.org/wiki/projections_of_population_growth)
				(hex!["563a7cd622c9faa1056b212542ab1bd68e22a80e4178e4c2dab794822c3d8158"].into(), 1 as u128),
				// faucet 2030 stash
				(hex!["1864b3de2f1250c4e2f5eb7044ede90c0b964b1914a197b44600dd4f07ea3026"].into(), 8_548_487_371 as u128),
				// faucet 2029 stash
				(hex!["52a331e8015fd252feae8db54b13c254ff11a800bdbbbac3444d0817484a2821"].into(), 8_477_660_723 as u128),
				// faucet 2028 stash
				(hex!["f0abd4e2e70e02285db5b2ce2c290431c3bda21357151dd84cea43ffcb168b4f"].into(), 8_405_863_301 as u128),
				// faucet 2027 stash
				(hex!["62399a6575b8b36e084c3dad0bf2a150ef27d06cf4697d27f974b36ee0385677"].into(), 8_333_078_318 as u128),
				// faucet 2026 stash
				(hex!["b02690e9c6654c8753cb9c4e3b60d4b223ceb5a30f25bdd1ff2cd3dffbc80824"].into(), 8_259_276_651 as u128),
				// faucet 2025 stash
				(hex!["02e62d4daaa5c7de58069b724c5072afd6bf81a73249ec9e4843d88aed44f165"].into(), 8_184_437_453 as u128),
				// faucet 2024 stash
				(hex!["5c405f5690d3bc456d08a38ddeafebca7606f8016e91a0256abf40b1866a1209"].into(), 8_108_605_255 as u128),
				// faucet 2023 stash
				(hex!["ee0c398fed6fae83588c7acff73d7c815b291f680a5182d74ad2a1bfbd59e375"].into(), 8_031_800_338 as u128),
				// faucet 2022 stash
				(hex!["665b3d89dc113e19d0c449619068a0aac149f706aaf9eb86e586bfc48ecb3f50"].into(), 7_953_952_577 as u128),
				// faucet 2021 stash
				(hex!["9e9c0de1dadbcc5d8f2fae0183e82ec2c07d3982ff93b507f69d81a1a6806e3f"].into(), 7_874_965_732 as u128),
			],
		),

		// bootnodes
    // todo: node key corrections required
		vec![
	    "/dns/trillian.snapr.com/tcp/30333/p2p/12D3KooWCT7rnUmEK7anTp7svwr4GTs6k3XXnSjmgTcNvdzWzgWU".parse().unwrap(),
	    "/dns/ford.snapr.net/tcp/30333/p2p/12D3KooWFHSc9cUcyNtavUkLg4VBAeBnYNgy713BnovUa9WNY5pp".parse().unwrap(),
	    "/dns/arthur.snapr.org/tcp/30333/p2p/12D3KooWAQqcXvcvt4eVEgogpDLAdGWgR5bY1drew44We6FfJAYq".parse().unwrap(),
		],

		// telemetry
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),

		// protocol id
		Some("snapr_mainnet"),

		// properties
		Some(snapr_properties()),

		// extensions
		Default::default(),
	))
}

fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {

	let evm_genesis_accounts = evm_genesis();

	const INITIAL_BALANCE: u128 = 100_000_000 * SNAPR;
	const INITIAL_STAKING: u128 =   1_000_000 * SNAPR;
	let existential_deposit = MaxNativeTokenExistentialDeposit::get();

	let balances = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), INITIAL_STAKING))
		.chain(endowed_accounts.iter().cloned().map(|k| (k, INITIAL_BALANCE)))
		.chain(
			get_all_module_accounts()
				.iter()
				.map(|x| (x.clone(), existential_deposit)),
		)
		.fold(
			BTreeMap::<AccountId, Balance>::new(),
			|mut acc, (account_id, amount)| {
				if let Some(balance) = acc.get_mut(&account_id) {
					*balance = balance
						.checked_add(amount)
						.expect("balance cannot overflow when building genesis");
				} else {
					acc.insert(account_id.clone(), amount);
				}
				acc
			},
		)
		.into_iter()
		.collect::<Vec<(AccountId, Balance)>>();

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_balances: Some(BalancesConfig { balances }),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (
						x.0.clone(), // stash
						x.0.clone(), // stash
						get_session_keys(
							x.2.clone(), // grandpa
							x.3.clone(), // babe
							x.4.clone(), // im-online
							x.5.clone(), // authority-discovery
						)))
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), INITIAL_STAKING, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: sp_runtime::Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_babe: Some(BabeConfig { authorities: vec![] }),
		pallet_grandpa: Some(GrandpaConfig { authorities: vec![] }),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_im_online: Default::default(),
		orml_tokens: Some(TokensConfig {
			endowed_accounts: endowed_accounts
				.iter()
				.flat_map(|x| {
					vec![
						(x.clone(), CurrencyId::Token(TokenSymbol::SEUR), INITIAL_BALANCE),
					]
				})
				.collect(),
		}),
		module_evm: Some(EvmConfig {
			accounts: evm_genesis_accounts,
		}),
		pallet_sudo: Some(SudoConfig { key: root_key }),
		pallet_collective_Instance1: Some(Default::default()),
	}
}

fn mainnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
) -> GenesisConfig {

	let evm_genesis_accounts = evm_genesis();

	const INITIAL_STAKING: u128 = 1_000_000 * SNAPR;
	let existential_deposit = MaxNativeTokenExistentialDeposit::get();

	let balances = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), INITIAL_STAKING*2))
		.chain(endowed_accounts.iter().cloned().map(|x| (x.0.clone(), x.1 * SNAPR)))
		.chain(
			get_all_module_accounts()
				.iter()
				.map(|x| (x.clone(), existential_deposit)),
		)
		.fold(
			BTreeMap::<AccountId, Balance>::new(),
			|mut acc, (account_id, amount)| {
				if let Some(balance) = acc.get_mut(&account_id) {
					*balance = balance
						.checked_add(amount)
						.expect("balance cannot overflow when building genesis");
				} else {
					acc.insert(account_id.clone(), amount);
				}
				acc
			},
		)
		.into_iter()
		.collect::<Vec<(AccountId, Balance)>>();

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_balances: Some(BalancesConfig { balances }),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (
						x.0.clone(), // stash
					  // todo:
					  // figure out why the stash account is provided twice here.
					  // should it be the controller account (x.1)?
					  // reef-chain git history may provide the answer
						x.0.clone(), // stash
						get_session_keys(
							x.2.clone(), // grandpa
							x.3.clone(), // babe
							x.4.clone(), // im-online
							x.5.clone(), // authority-discovery
						)))
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), INITIAL_STAKING, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: sp_runtime::Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_babe: Some(BabeConfig { authorities: vec![] }),
		pallet_grandpa: Some(GrandpaConfig { authorities: vec![] }),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_im_online: Default::default(),
		orml_tokens: Some(TokensConfig {
			endowed_accounts: vec![]
		}),
		module_evm: Some(EvmConfig {
			accounts: evm_genesis_accounts,
		}),
		pallet_sudo: Some(SudoConfig { key: root_key }),
		pallet_collective_Instance1: Some(Default::default()),
	}
}


/// Token
pub fn snapr_properties() -> Properties {
	let mut p = Properties::new();
	p.insert("ss58format".into(), 42.into());
	p.insert("tokenDecimals".into(), 18.into());
	p.insert("tokenSymbol".into(), "SNAPR".into());
	p
}


/// Predeployed contract addresses
pub fn evm_genesis() -> BTreeMap<H160, module_evm::GenesisAccount<Balance, Nonce>> {
	let existential_deposit = MaxNativeTokenExistentialDeposit::get();
	let contracts_json = &include_bytes!("../../assets/bytecodes.json")[..];
	let contracts: Vec<(String, String, String)> = serde_json::from_slice(contracts_json).unwrap();
	let mut accounts = BTreeMap::new();
	for (_, address, code_string) in contracts {
		let account = module_evm::GenesisAccount {
			nonce: 0,
			balance: existential_deposit,
			storage: Default::default(),
			code: Bytes::from_str(&code_string).unwrap().0,
		};
		let addr = H160::from_slice(
			from_hex(address.as_str())
				.expect("predeploy-contracts must specify address")
				.as_slice(),
		);
		accounts.insert(addr, account);
	}
	accounts
}
