#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		ensure,
		pallet_prelude::*,
		sp_runtime::traits::{AccountIdConversion, Hash, Saturating, Zero},
		storage::child,
		traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons},
		PalletId,
	};
	use frame_system::{pallet_prelude::*, ensure_signed};
	use super::*;

	pub type FundIndex = u32;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type SubmissionDeposit: Get<BalanceOf<Self>>;
		type MinContribution: Get<BalanceOf<Self>>;
		type RetirementPeriod: Get<Self::BlockNumber>;
	}
	#[derive(Encode, Decode, Default, PartialEq, Eq, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct FundInfo<AccountId, Balance, BlockNumber> {
		/// The account that will recieve the funds if the campaign is successful.
		beneficiary: AccountId,
		/// The amount of deposit placed.
		deposit: Balance,
		/// The total amount raised.
		raised: Balance,
		/// Block number after which funding must have succeeded.
		end: BlockNumber,
		/// Upper bound on `raised`.
		goal: Balance,
	}
	type FundInfoOf<T> =
	FundInfo<AccountIdOf<T>, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

	// #[pallet::storage]
	// #[pallet::getter(fn funds)]
	// /// Info on all of the funds.
	// pub(super) type Funds<T: Config> = StorageMap<
	// 	_,
	// 	Blake2_128Concat,
	// 	FundIndex,
	// 	FundInfoOf<T>,
	// 	OptionQuery,
	// >;

	// #[pallet::storage]
	// #[pallet::getter(fn fund_count)]
	// /// The total number of funds that have so far been allocated.
	// pub(super) type FundCount<T: Config> = StorageValue<_, FundIndex, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		// pub fn fund_account_id(index: FundIndex) -> T::AccountId {
		// 	const PALLET_ID: ModuleId = ModuleId(*b"ex/cfund");
		// 	PALLET_ID.into_sub_account(index)
		// }
		// pub fn id_from_index(index: FundIndex) -> child::ChildInfo {
		// 	let mut buf = Vec::new();
		// 	buf.extend_from_slice(b"crowdfnd");
		// 	buf.extend_from_slice(&index.to_le_bytes()[..]);
	
		// 	child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
		// }
	}
   
}
