#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_system::pallet::*;
pub use frame_support::storage::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn key)]
	pub type IssuedKeys<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, Vec<u8>, Vec<u8>>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KeyIssued(Vec<u8>, T::AccountId),
		KeyRevoked(Vec<u8>, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn issue_key(origin: OriginFor<T>, fingerprint: Vec<u8>, hash: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<IssuedKeys<T>>::insert(&who, &fingerprint, hash);

			Self::deposit_event(Event::KeyIssued(fingerprint, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn revoke_key(origin: OriginFor<T>, key_index: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<IssuedKeys<T>>::remove(&who, &key_index);

			Self::deposit_event(Event::KeyRevoked(key_index, who));
			Ok(())
		}
	}
}
