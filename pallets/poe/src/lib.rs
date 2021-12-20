#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/*#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
*/
#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// 存证的最大长度
		type MaxClaimLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T:Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;


	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimTransfered(T::AccountId,T::AccountId, Vec<u8>), //存证转移

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		///存证已经存在
		ProofAlreadyExist,
		///存证不存在
		ProofNotExist,
		///不是存证的拥有者
		NotClaimOwner,
		///存证太长
		ClaimTooLarge,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		///
		/// 创建存证
		#[pallet::weight(0)]
		//#[pallet::weight(T::WeightInfo::create_claim_benchmark(claim.to_vec()))]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			/*let claim_len=claim.len();
			println!("{}",claim_len);
			if claim_len>10 {
				Err(Error::<T>::ClaimTooLarge)?;
			}*/

			//ensure!(claim.len()<=10, Error::<T>::ClaimTooLarge);

			ensure!(
                T::MaxClaimLength::get() >= claim.len() as u32,
                Error::<T>::ClaimTooLarge
            );

			Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimCreated(sender,claim));

			Ok(().into())

		}

		///
		/// 撤销存证
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			//使用ok_or,这里注销掉
			//ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

			let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofNotExist)?;

			ensure!(owner==sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(sender,claim));

			Ok(().into())

		}

		///转移存证
		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, target:T::AccountId, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			//使用ok_or,这里注销掉
			//ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

			let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofNotExist)?;

			ensure!(owner==sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Proofs::<T>::insert(&claim, (target.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimTransfered(sender,target,claim));

			Ok(().into())

		}
	}
}
