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
		dispatch::DispatchResult,
		traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
	};
	use frame_system::pallet_prelude::*;
	use frame_system::ensure_signed;

	const EXAMPLE_ID: LockIdentifier = *b"example ";

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type StakeCurrency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
		type BalanceOf<T> =
		<<T as Config>::StakeCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Locked(T::AccountId, BalanceOf<T>),
		Unlocked(T::AccountId),
		LockExtended(T::AccountId, BalanceOf<T>),
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
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub(super) fn lock_capital(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>
		) -> DispatchResultWithPostInfo {
	
			let user = ensure_signed(origin)?;
	
			T::StakeCurrency::set_lock(
				EXAMPLE_ID,
				&user,
				amount,
				WithdrawReasons::all(),
			);
	
			Self::deposit_event(Event::Locked(user, amount));
			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub(super) fn extend_lock(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;
	
			T::StakeCurrency::extend_lock(
				EXAMPLE_ID,
				&user,
				amount,
				WithdrawReasons::all(),
			);
	
			Self::deposit_event(Event::ExtendedLock(user, amount));
			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub(super) fn unlock_all(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;
	
			T::StakeCurrency::remove_lock(EXAMPLE_ID, &user);
	
			Self::deposit_event(Event::Unlocked(user));
			Ok(().into())
		}
		
	}
}
