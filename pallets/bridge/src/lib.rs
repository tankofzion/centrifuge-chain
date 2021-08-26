// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! # Bridge pallet
//!
//! This pallet implements bla bla bla
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//! This pallet is used for bridging chains.
//!
//! ## Terminology
//!
//! ## Usage
//!
//! ## Interface
//!
//! ### Supported Origins
//!
//! Signed origin is valid.
//!
//! ### Types
//!
//! `BridgeOrigin` - Specifies the origin check provided by the chainbridge for calls that can only be called by the chainbridge pallet.
//! `AdminOrigin` -
//! `BridgePalletId` - Identifier of the pallet (parameterized when the pallet is built for a runtime).
//! `Currency` - Currency as viewed from this pallet.
//! `Event` - Type for events triggered by this pallet.
//! `NativeTokenId` - Identifier of the native token.
//! `TokenTransferFee` - Additional fee charged for transfering tokens.
//! `NftTransferFee` - Additional fee charged when moving NFTs to target chains.
//! `WeightInfo` - Weight information for extrinsics in this pallet.
//!
//! ### Events
//!
//! `Remark` - Event triggered when ...
//!
//! ### Errors
//! `ResourceIdDoesNotExist` - Resource id provided on initiating a transfer is not a key in bridges-names mapping.
//! `RegistryIdDoesNotExist` - Registry id provided on receiving a transfer is not a key in bridges-names mapping.
//! `InvalidTransfer` - Invalid transfer.
//! `InsufficientBalance` - Not enough resources/assets for performing a transfer.
//!
//! ### Dispatchable Functions
//!
//! Callable functions (or extrinsics), also considered as transactions, materialize the
//! pallet contract. Here's the callable functions implemented in this module:
//!
//! [`receive_nonfungible`]
//! [`remark`]
//! [`transfer`]
//! [`transfer_asset`]
//! [`transfer_native`]
//!
//! ### Public Functions
//!
//! ## Genesis Configuration
//! This pallet depends on the [`GenesisConfig`]. The following fields are added to
//! the genesis configuration, that are not associated with specific storage values:
//! `chains: Vec<u8>` - List of chains.
//! `relayers: Vec<T::AccountId>`- List of relayers.
//! `resources: Vec<(ResourceId, Vec<u8>)>` - List of resources (or assets).
//! `threshold: u32` - Threshold value.
//!
//! ## Related Pallets
//! This pallet is tightly coupled to the following pallets:
//! - Substrate FRAME's [`balances` pallet](https://github.com/paritytech/substrate/tree/master/frame/balances).
//! - Centrifuge Chain [`bridge_mapping` pallet](https://github.com/centrifuge/centrifuge-chain/tree/master/pallets/bridge-mapping).
//! - Centrifuge Chain [`chainbrige` pallet](https://github.com/centrifuge/chainbridge-substrate).
//! - Centrifuge Chain [`fees` pallet](https://github.com/centrifuge/centrifuge-chain/tree/master/pallets/fees).
//! - Centrifuge Chain [`nft` pallet](https://github.com/centrifuge/centrifuge-chain/tree/master/pallets/nft).
//! - Centrifuge Chain [`registry` pallet](https://github.com/centrifuge/centrifuge-chain/tree/master/pallets/registry).
//!
//! ## References
//! - [Substrate FRAME v2 attribute macros](https://crates.parity.io/frame_support/attr.pallet.html).
//!
//! ## Credits
//! The Centrifugians Tribe <tribe@centrifuge.io>
//!
//! ## License
//! GNU General Public License, Version 3, 29 June 2007 <https://www.gnu.org/licenses/gpl-3.0.html>

// Ensure we're `no_std` when compiling for WebAssembly.
#![cfg_attr(not(feature = "std"), no_std)]

// ----------------------------------------------------------------------------
// Module imports and re-exports
// ----------------------------------------------------------------------------

// Mock runtime and unit test cases
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// Pallet types and traits
pub mod traits;
pub mod types;

// Pallet extrinsics weight information
mod weights;

// Centrifuge chain common types
use centrifuge_commons::types::{AssetId, RegistryId, TokenId};

use chainbridge::types::{ChainId, ResourceId};

use codec::FullCodec;

use core::convert::TryInto;

// Runtime, system and frame primitives
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	inherent::Vec,
	traits::{Currency, EnsureOrigin, ExistenceRequirement::AllowDeath, Get, WithdrawReasons},
	transactional, PalletId,
};

use frame_system::{ensure_root, pallet_prelude::OriginFor};

use sp_core::U256;

use sp_runtime::{
	sp_std::vec,
	traits::{AccountIdConversion, CheckedAdd, CheckedSub, SaturatedConversion},
};

use sp_std::prelude::*;

use unique_assets::traits::Unique;

use crate::types::{Address, Bytes32};

// Extrinsics weight information
use crate::traits::WeightInfo;

// Re-export in crate namespace (for runtime construction)
pub use pallet::*;

// ----------------------------------------------------------------------------
// Type aliases
// ----------------------------------------------------------------------------

type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// ----------------------------------------------------------------------------
// Pallet module
// ----------------------------------------------------------------------------

// Bridge pallet module
//
// The name of the pallet is provided by `construct_runtime` and is used as
// the unique identifier for the pallet's storage. It is not defined in the
// pallet itself.
#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Bridge pallet type declaration.
	//
	// This structure is a placeholder for traits and functions implementation
	// for the pallet.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ------------------------------------------------------------------------
	// Pallet configuration
	// ------------------------------------------------------------------------

	/// Bridge pallet's configuration trait.
	///
	/// Associated types and constants are declared in this trait. If the pallet
	/// depends on other super-traits, the latter must be added to this trait,
	/// such as, in this case, [`pallet_nft::Config`] super-trait, for instance.
	/// Note that [`frame_system::Config`] must always be included.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_balances::Config
		+ pallet_bridge_mapping::Config
		+ pallet_fees::Config
		+ pallet_nft::Config
	{
		/// Specifies the origin check provided by the chainbridge for calls
		/// that can only be called by the chainbridge pallet.
		type BridgeOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

		/// Constant configuration parameter to store the module identifier for the pallet.
		///
		/// The module identifier may be of the form ```PalletId(*b"rd/claim")``` and set
		/// using the [`parameter_types`](https://substrate.dev/docs/en/knowledgebase/runtime/macros#parameter_types)
		// macro in the [`runtime/lib.rs`] file.
		#[pallet::constant]
		type BridgePalletId: Get<PalletId>;

		/// Resource identifier type
		type ResourceId: Member + Default + FullCodec + Into<[u8; 32]> + From<[u8; 32]>;

		type AdminOrigin: EnsureOrigin<Self::Origin>;

		/// Currency as viewed from this pallet
		type Currency: Currency<Self::AccountId>;

		/// Associated type for Event enum
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type NativeTokenId: Get<<Self as pallet::Config>::ResourceId>;

		/// Additional fee charged when moving native tokens to target chains.
		#[pallet::constant]
		type TokenTransferFee: Get<u128>;

		/// Additional fee charged when moving NFTs to target chains.
		#[pallet::constant]
		type NftTransferFee: Get<u128>;

		/// Weight information for extrinsics in this pallet
		type WeightInfo: WeightInfo;
	}

	// ------------------------------------------------------------------------
	// Pallet events
	// ------------------------------------------------------------------------

	// The macro generates event metadata and derive Clone, Debug, Eq, PartialEq and Codec
	#[pallet::event]
	// The macro generates a function on Pallet to deposit an event
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Remark(<T as frame_system::Config>::Hash, ResourceId),
	}

	// ------------------------------------------------------------------------
	// Pallet storage items
	// ------------------------------------------------------------------------

	// Additional fee charged when moving native tokens to target chains.
    // FIXME: Now token transfer fee is set as a runtime parameter. If we agree with this, this method
    //        can be removed.
    // #[pallet::storage]
	// #[pallet::getter(fn current_slot)]
	// pub type TokenTransferFee<T> = StorageValue<_, u128, ValueQuery>;

	// ------------------------------------------------------------------------
	// Pallet genesis configuration
	// ------------------------------------------------------------------------

	// The genesis configuration type.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub chains: Vec<u8>,
		pub relayers: Vec<T::AccountId>,
		pub resources: Vec<(ResourceId, Vec<u8>)>,
		pub threshold: u32,
	}

	// The default value for the genesis config type.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				chains: Default::default(),
				relayers: Default::default(),
				resources: Default::default(),
				threshold: Default::default(),
			}
		}
	}

	// The build of genesis for the pallet.
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<Pallet<T>>::initialize(
				&self.chains,
				&self.relayers,
				&self.resources,
				&self.threshold,
			);
		}
	}

	// ------------------------------------------------------------------------
	// Pallet lifecycle hooks
	// ------------------------------------------------------------------------

    // Additional 
    // FIXME: Now token transfer fee is set as a runtime parameter. If we agree with this, this method
    //        can be removed.

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// ------------------------------------------------------------------------
	// Pallet errors
	// ------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		/// Resource id provided on initiating a transfer is not a key in bridges-names mapping.
		ResourceIdDoesNotExist,

		/// Registry id provided on recieving a transfer is not a key in bridges-names mapping.
		RegistryIdDoesNotExist,

		/// Invalid transfer
		InvalidTransfer,

		/// Not enough means for performing a transfer
		InsufficientBalance,

		/// Token transfer fee not convertible to currrency
		TokenTransferFeeNotConvertibleToCurrency,

		/// Total amount not convertible to currrency
		TotalAmountNotConvertibleToCurrency,
	}

	// ------------------------------------------------------------------------
	// Pallet dispatchable functions
	// ------------------------------------------------------------------------

	// Declare Call struct and implement dispatchable (or callable) functions.
	//
	// Dispatchable functions are transactions modifying the state of the chain. They
	// are also called extrinsics are constitute the pallet's public interface.
	// Note that each parameter used in functions must implement `Clone`, `Debug`,
	// `Eq`, `PartialEq` and `Codec` traits.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfer an nft to a whitelisted destination chain. Source nft is locked in bridge account
		/// rather than being burned.
		#[pallet::weight(<T as Config>::WeightInfo::transfer_asset())]
		#[transactional]
		pub fn transfer_asset(
			origin: OriginFor<T>,
			recipient: Vec<u8>,
			from_registry: RegistryId,
			token_id: TokenId,
			dest_id: ChainId,
		) -> DispatchResultWithPostInfo {
			let source = ensure_signed(origin)?;

			// Get resource id from registry
			let reg: Address = from_registry.into();
			let reg: Bytes32 = reg.into();
			let reg: <T as pallet_bridge_mapping::Config>::Address = reg.into();
			let resource_id = <pallet_bridge_mapping::Pallet<T>>::name_of(reg)
				.ok_or(Error::<T>::ResourceIdDoesNotExist)?;

			// Charge additional fees for NFT transfer
			<pallet_fees::Pallet<T>>::burn_fee(&source, T::NftTransferFee::get().saturated_into())?;

			// Lock asset by transfering to bridge account
			let bridge_id = <chainbridge::Pallet<T>>::account_id();
			let asset_id = AssetId(from_registry, token_id);
			<pallet_nft::Pallet<T> as Unique>::transfer(&source, &bridge_id, &asset_id)?;

			// Transfer instructions for relayer
			let tid: &mut [u8] = &mut [0; 32];

			// Ethereum is big-endian
			token_id.to_big_endian(tid);
			<chainbridge::Pallet<T>>::transfer_nonfungible(
				dest_id,
				resource_id.into(),
				tid.to_vec(),
				recipient,
				vec![],
			)?;

			Ok(().into())
		}

		/// Transfers some amount of the native token to some recipient on a (whitelisted) destination chain.
		#[pallet::weight(<T as Config>::WeightInfo::transfer_native())]
        #[transactional]
		pub fn transfer_native(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			recipient: Vec<u8>,
			dest_id: ChainId,
		) -> DispatchResultWithPostInfo {
			let source = ensure_signed(origin)?;

			let token_fee: T::Balance = T::TokenTransferFee::get().saturated_into();
			let currency_token_fee: BalanceOf<T> = TryInto::<u128>::try_into(token_fee)
				.map_err(|_| Error::<T>::TokenTransferFeeNotConvertibleToCurrency)?
				.try_into()
				.map_err(|_| Error::<T>::TokenTransferFeeNotConvertibleToCurrency)?;
			let total_amount = amount
				.checked_add(&currency_token_fee)
				.ok_or(Error::<T>::TotalAmountNotConvertibleToCurrency)?;

			// Ensure account has enough balance for both fee and transfer
			// Check to avoid balance errors down the line that leave balance storage in an inconsistent state
			let remaining_balance = <T as pallet::Config>::Currency::free_balance(&source)
				.checked_sub(&total_amount)
				.ok_or(Error::<T>::InsufficientBalance)?;
			<T as pallet::Config>::Currency::ensure_can_withdraw(
				&source,
				total_amount,
				WithdrawReasons::all(),
				remaining_balance,
			)
			.map_err(|_| Error::<T>::InsufficientBalance)?;

			ensure!(
				<chainbridge::Pallet<T>>::chain_whitelisted(dest_id),
				Error::<T>::InvalidTransfer
			);

			// Burn additional fees
			<pallet_fees::Pallet<T>>::burn_fee(
				&source,
				T::TokenTransferFee::get().saturated_into(),
			)?;

			let bridge_id = <chainbridge::Pallet<T>>::account_id();
			<T as pallet::Config>::Currency::transfer(
				&source,
				&bridge_id,
				amount.into(),
				AllowDeath,
			)?;

			let resource_id = T::NativeTokenId::get();
			<chainbridge::Pallet<T>>::transfer_fungible(
				dest_id,
				resource_id.into(),
				recipient,
                // Note: use u128 to restrict balance greater than 128bits
            U256::from(amount.saturated_into::<u128>()),
			)?;

			Ok(().into())
		}

		/// Executes a simple currency transfer using the chainbridge account as the source
		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
        #[transactional]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			_r_id: ResourceId,
		) -> DispatchResultWithPostInfo {
			let source = T::BridgeOrigin::ensure_origin(origin)?;
			<T as pallet::Config>::Currency::transfer(&source, &to, amount.into(), AllowDeath)?;

			Ok(().into())
		}

		/// Receive a non-fungbile token
		#[pallet::weight(<T as Config>::WeightInfo::receive_nonfungible())]
		pub fn receive_nonfungible(
			origin: OriginFor<T>,
			to: T::AccountId,
			token_id: TokenId,
			_metadata: Vec<u8>,
			resource_id: ResourceId,
		) -> DispatchResultWithPostInfo {
			let source = T::BridgeOrigin::ensure_origin(origin)?;

			// Get registry from resource id
			let rid: <T as pallet_bridge_mapping::Config>::ResourceId = resource_id.into();
			let registry_id = <pallet_bridge_mapping::Pallet<T>>::addr_of(rid)
				.ok_or(Error::<T>::RegistryIdDoesNotExist)?;
			let registry_id: Address = registry_id.into().into();

			// Transfer from bridge account to destination account
			let asset_id = AssetId(registry_id.into(), token_id);
			<pallet_nft::Pallet<T> as Unique>::transfer(&source, &to, &asset_id)?;

			Ok(().into())
		}

		/// This can be called by the chainbridge to demonstrate an arbitrary call from a proposal.
		#[pallet::weight(<T as Config>::WeightInfo::remark())]
		pub fn remark(
			origin: OriginFor<T>,
			hash: T::Hash,
			r_id: ResourceId,
		) -> DispatchResultWithPostInfo {
			T::BridgeOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::Remark(hash, r_id));

			Ok(().into())
		}


        // Update token transfer fee
        // FIXME: Now token transfer fee is set as a runtime parameter. If we agree with this, this method
        //        can be removed.

		// #[pallet::weight(<T as Config>::WeightInfo::set_token_transfer_fee())]
		// pub fn set_token_transfer_fee(
        //     origin: OriginFor<T>, 
        //     fee: T::Balance
        // ) -> DispatchResultWithPostInfo {
		// 	Self::ensure_admin(origin)?;
		// 	TokenTransferFee::<T>::mutate(|transfer_token_fee| {
		// 		*transfer_token_fee = fee
		// 	});
		// 	Ok(().into())
		// }
    }
} // end of 'pallet' module

// ----------------------------------------------------------------------------
// Pallet implementation block
// ----------------------------------------------------------------------------

// Pallet implementation block.
//
// This main implementation block contains two categories of functions, namely:
// - Public functions: These are functions that are `pub` and generally fall into
//   inspector functions that do not write to storage and operation functions that do.
// - Private functions: These are private helpers or utilities that cannot be called
//   from other pallets.
impl<T: Config> Pallet<T> {
	/// Return the account identifier of the RAD claims pallet.
	///
	/// This actually does computation. If you need to keep using it, then make
	/// sure you cache the value and only call this once.
	pub fn account_id() -> T::AccountId {
		T::BridgePalletId::get().into_account()
	}

	/// Initialize pallet's genesis configuration.
	///
	/// This private helper function is used for setting pallet's genesis
	/// configuration up.
	fn initialize(
		chains: &[u8],
		relayers: &[T::AccountId],
		resources: &Vec<(ResourceId, Vec<u8>)>,
		threshold: &u32,
	) {
		chains.into_iter().for_each(|c| {
			<chainbridge::Pallet<T>>::whitelist(*c).unwrap_or_default();
		});
		relayers.into_iter().for_each(|rs| {
			<chainbridge::Pallet<T>>::register_relayer(rs.clone()).unwrap_or_default();
		});

		<chainbridge::Pallet<T>>::set_relayer_threshold(*threshold).unwrap_or_default();

		for &(ref re, ref m) in resources.iter() {
			<chainbridge::Pallet<T>>::register_resource(*re, m.clone()).unwrap_or_default();
		}
	}

    // Ensure that the caller has admin rights
    //
    // FIXME: Now token transfer fee is set as a runtime parameter. If we agree with this, this method
    //        can be removed.
    // fn ensure_admin(origin: OriginFor<T>) -> DispatchResult {
	// 	<T as Config>::AdminOrigin::try_origin(origin)
	// 		.map(|_| ())
	// 		.or_else(ensure_root)?;
	// 	Ok(())
    // }
}
