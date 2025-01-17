//! Testnet fixtures

use dkg_runtime_primitives::crypto::AuthorityId as DKGId;
use dkg_standalone_runtime::AccountId;
use hex_literal::hex;
use sc_network::config::MultiaddrWithPeerId;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::UncheckedInto;
use sp_finality_grandpa::AuthorityId as GrandpaId;

/// Testnet root key
pub fn get_testnet_root_key() -> AccountId {
	// Arana sudo key: 5F9jS22zsSzmWNXKt4kknBsrhVAokEQ9e3UcuBeg21hkzqWz
	return hex!["888a3ab33eea2b827f15302cb26af0e007b067ccfbf693faff3aa7ffcfa25925"].into()
}

/// Arana bootnodes
pub fn get_arana_bootnodes() -> Vec<MultiaddrWithPeerId> {
	return vec![
		"/ip4/158.247.202.146/tcp/30333/p2p/12D3KooWJ7PSFvtU3i1LK8E6QmPqHvihs9RTvFw9ZoxGpHGhgNG3"
			.parse()
			.unwrap(),
		"/ip4/158.247.210.6/tcp/30333/p2p/12D3KooWQs9faHSzWq4EadVmaBmqTDV2KmEBVQdaXDQLccd9bRjS"
			.parse()
			.unwrap(),
		"/ip4/141.164.55.77/tcp/30333/p2p/12D3KooWSBjUxtAxw4Jpb76QY4JySsrr5seB4QhkYVB64sVWbAFH"
			.parse()
			.unwrap(),
	]
}

/// Arana initial authorities
pub fn get_arana_initial_authorities() -> Vec<(AccountId, AccountId, AuraId, GrandpaId, DKGId)> {
	return vec![
		(
			hex!["4e85271af1330e5e9384bd3ac5bdc04c0f8ef5a8cc29c1a8ae483d674164745c"].into(),
			hex!["804808fb75d16340dc250871138a1a6f1dfa3cab9cc1fbd6f42960f1c39a950d"].into(),
			hex!["16be9647f91aa5441e300acb8f0d6ccc63e72850202a7947df6a646c1bb4071a"]
				.unchecked_into(),
			hex!["71bf01524c555f1e0f6b7dc7243caf00851d3afc543422f98d3eb6bca78acd8c"]
				.unchecked_into(),
			hex!["028a4c0781f8369fdd873f8531491f24e2e806fd11a13d828cb4099e6c1045103e"]
				.unchecked_into(),
		),
		(
			hex!["587c2ef00ec0a1b98af4c655763acd76ece690fccbb255f01663660bc274960d"].into(),
			hex!["cc195602a63bbdcf2ef4773c86fdbfefe042cb9aa8e3059d02e59a062d9c3138"].into(),
			hex!["f4e206607ffffcd389c4c60523de5dda5a411d1435f8540b6b6bc181553bd65a"]
				.unchecked_into(),
			hex!["61f771ebfdb0a6de08b8e0ca7a39a01f24e7eaa3d1e7f1001e6503490c25c044"]
				.unchecked_into(),
			hex!["02427a6cf7f1d7538d9e3e4df834e27db337fd6ef0f530aab4e9799ff865e843fc"]
				.unchecked_into(),
		),
		(
			hex!["368ea402dbd9c9888ae999d6a799cf36e08673ee53c001dfb4529c149fc2c13b"].into(),
			hex!["a24f729f085de51eebaeaeca97d6d499761b8f6daeca9b99d754a06ef8bcec3f"].into(),
			hex!["8e92157e55a72fe0ee78c251a7553af341635bec0aafee1e4189cf8ce52cdd71"]
				.unchecked_into(),
			hex!["a41a815db90b9bd3d9ec462f90ba77ba1d627a9fccc9f7847e34c9e9e9b57c90"]
				.unchecked_into(),
			hex!["036aec5853fba2662f31ba89e859ac100daa6c58dc8fdaf0555565663f2b99f8f2"]
				.unchecked_into(),
		),
	]
}
