#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

#[cfg(test)]
pub mod mock;

#[cfg(test)]
pub mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;


	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The maximum length of claim that can be added.
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		/// The runtime event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}


	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)] // 设置交易的权重，这里设置为 0
		pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?; // 确保交易的发送者是已经签名的

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist); // 确保该声明不存在

			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()), // 将声明和发送者以及区块号存储到链上
			);

			Self::deposit_event(Event::ClaimCreated(sender, claim)); // 触发声明创建事件

			Ok(().into()) // 返回成功的交易结果
		}

		#[pallet::weight(0)] // 设置交易的权重，这里设置为 0
		pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?; // 确保交易的发送者是已经签名的

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			// 获取声明的所有者和区块号，如果声明不存在则返回错误
			ensure!(owner == sender, Error::<T>::NotClaimOwner); // 确保发送者是声明的所有者

			Proofs::<T>::remove(&claim); // 从链上移除该声明

			Self::deposit_event(Event::ClaimRevoked(sender, claim)); // 触发声明撤销事件

			Ok(().into()) // 返回成功的交易结果
		}


		#[pallet::weight(0)] // 设置交易的权重，这里设置为 0
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
			dest: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?; // 确保交易的发送者是已经签名的

			let (owner, _block_number) =
				Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			// 获取声明的所有者和区块号，如果声明不存在则返回错误
			ensure!(owner == sender, Error::<T>::NotClaimOwner); // 确保发送者是声明的所有者

			Proofs::<T>::insert(&claim, (dest, frame_system::Pallet::<T>::block_number())); // 将声明的所有权转移给目标账户

			Ok(().into()) // 返回成功的交易结果
		}
	}
}
