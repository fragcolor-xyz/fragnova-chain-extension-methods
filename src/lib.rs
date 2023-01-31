//! This crate contains the definitions of all the chain extension methods of the Fragnova Blockchain.
//!
//! Fragnova ink! smart contract developers should integrate these method definitons into the environment of their smart contracts (https://paritytech.github.io/ink/ink/attr.chain_extension.html#example-environment),
//! if they wish to call these methods in their smart contracts.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_env::{AccountId, Environment, DefaultEnvironment};
use ink_prelude::vec::Vec;

use sp_fragnova::{
	Hash128,
	Hash256,
	protos::{
		Proto
	},
	fragments::{
		FragmentDefinition,
		FragmentInstance,
		InstanceUnit
	}
};

type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
type AssetId = u64;

/// `#[ink::chain_extension]` defines the interface for a chain extension.
///
/// The interface consists of an error code that indicates lightweight errors as well as the definition of some chain extension methods.
/// The overall structure follows that of a simple Rust trait definition. The error code is defined as an associated type definition of the trait definition.
/// The methods are defined as associated trait methods without implementation.
///
/// Chain extension methods must not have a `self` receiver such as `&self` or `&mut self` and
/// must have inputs and output that implement the SCALE encoding and decoding.
/// Their return value follows specific rules that can be altered using the `handle_status` attribute.
///
/// Source: https://paritytech.github.io/ink/ink/attr.chain_extension.html#
#[ink::chain_extension]
pub trait MyChainExtension {
	/// Error codes of the chain extension.
	///
	/// By default (i.e unless both the `return_result` and `handle_status` attributes are `false`)
	/// all chain extension methods should return a `Result<T, E>` where `E: From<Self::ErrorCode>`.
	/// The `Self::ErrorCode` represents the error code of the chain extension.
	/// This means that a smart contract calling such a chain extension method first queries
	/// the returned status code of the chain extension method and
	/// **only loads and decodes the output if the returned status code indicates a successful call.**
	///
	/// A chain extension method that is flagged with `handle_status = false` assumes that the returned error code will always indicate success.
	/// Therefore it will always load and decode the output buffer and loses the `E: From<Self::ErrorCode>` constraint for the call.
	///
	/// Source: https://paritytech.github.io/ink/ink/attr.chain_extension.html
	type ErrorCode = MyChainExtensionError;

	/// Get the `Proto` struct of the Proto-Fragment which has an ID of `proto_hash`
	#[ink(extension = 0x0b01, handle_status = false, returns_result = false)]
	/// Get the list of Proto-Fragments that are owned by `owner`
	fn get_proto(proto_hash: Hash256) -> Option<Proto<AccountId, BlockNumber>>;
	#[ink(extension = 0x0b02, handle_status = false, returns_result = false)]
	fn get_proto_ids(owner: AccountId) -> Vec<Hash256>;

	/// Get the `FragmentDefinition` struct of the Fragment Definition which has the ID of `definition_hash`
	#[ink(extension = 0x0c01, handle_status = false, returns_result = false)]
	fn get_definition(definition_hash: Hash128) -> Option<FragmentDefinition<Vec<u8>, AssetId, AccountId, BlockNumber>>;
	/// Get the `FragmentInstance` struct of the Fragment Instance whose Fragment Definition ID is `definition_hash`,
	/// whose Edition ID is `edition_id` and whose Copy ID is `copy_id`
	#[ink(extension = 0x0c02, handle_status = false, returns_result = false)]
	fn get_instance(definition_hash: Hash128, edition_id: InstanceUnit, copy_id: InstanceUnit) -> Option<FragmentInstance<BlockNumber>>;
	/// Get the list of Fragment Instances that are owned by `owner`
	#[ink(extension = 0x0c03, handle_status = false, returns_result = false)]
	fn get_instance_ids(owner: AccountId) -> Vec<(Hash128, InstanceUnit, InstanceUnit)>;
	/// Give a Fragment Instance (that is owned by the smart contract) to `to`.
	#[ink(extension = 0x0c04, returns_result = false)]
	fn give_instance(definition_hash: Hash128, edition_id: InstanceUnit, copy_id: InstanceUnit, to: AccountId);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MyChainExtensionError {
	NotFound,
	NoPermission,
}

impl From<scale::Error> for MyChainExtensionError {
	fn from(_: scale::Error) -> Self {
		panic!("encountered unexpected invalid SCALE encoding")
	}
}

/// The defined `ErrorCode` must implement `FromStatusCode` which should be implemented as a
/// more or less trivial conversion from the `u32` status code to a `Result<(), Self::ErrorCode>`.
/// The `Ok(())` value indicates that the call to the chain extension method was successful.
///
/// By convention an error code of `0` represents success.
/// However, chain extension authors may use whatever suits their needs.
///
/// Source: https://paritytech.github.io/ink/ink/attr.chain_extension.html#error-code
impl ink_env::chain_extension::FromStatusCode for MyChainExtensionError {
	fn from_status_code(status_code: u32) -> Result<(), Self> {
		match status_code {
			0 => Ok(()),
			1 => Err(Self::NotFound),
			2 => Err(Self::NoPermission),
			_ => panic!("encountered unknown status code"),
		}
	}
}



