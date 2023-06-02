// 如果没有启用 "std" 特性，则不使用标准库
#![cfg_attr(not(feature = "std"), no_std)]

// 导出 pallet 模块
pub use pallet::*;

// 导入 frame_support 库中的 pallet_prelude 模块中的所有内容
use frame_support::pallet_prelude::*;

// 导入 frame_system 库中的 pallet_prelude 模块中的所有内容
use frame_system::pallet_prelude::*;

// 导入 sp_std 库中的 prelude 模块中的所有内容
use sp_std::prelude::*;

// 如果是测试环境，则导入 mock 和 tests 模块
#[cfg(test)]
pub mod mock;

#[cfg(test)]
pub mod tests;

// 定义 pallet 模块
#[frame_support::pallet]
pub mod pallet {
	// 导入 frame_support 库中的 pallet_prelude 模块中的所有内容
	use frame_support::pallet_prelude::*;

	// 导入 frame_system 库中的 pallet_prelude 模块中的所有内容
	use frame_system::pallet_prelude::*;

	// 导入 frame_support 库中的 traits 模块中的 Randomness 特性
	use frame_support::traits::Randomness;

	// 导入 sp_io 库中的 hashing 模块中的 blake2_128 函数
	use sp_io::hashing::blake2_128;

	// 定义 kittyId 类型为 u32
	pub type kittyId = u32;

	// 定义 GetDefaultValue 函数，返回值为 kittyId 类型的 0
	#[pallet::type_value]
	pub fn GetDefaultValue() -> kittyId {
		0_u32
	}

	// 定义 Kitty 结构体，包含一个长度为 16 的 u8 数组
	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub [u8; 16]);

	// 定义 Config 特性，继承 frame_system::Config 特性
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// 定义 Event 类型，从 Event<Self> 转换而来，同时也是 frame_system::Config 的 Event 类型
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// 定义 Randomness 特性，使用 Self::Hash 和 Self::BlockNumber 作为泛型参数
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// 定义 Pallet 结构体，使用泛型参数 T
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	// 定义 NextKittyId 存储，使用泛型参数 T，存储类型为 kittyId，查询类型为 ValueQuery，初始值为 GetDefaultValue 函数的返回值
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T> = StorageValue<_, kittyId, ValueQuery, GetDefaultValue>;

	// 定义 Kitties 存储，使用泛型参数 T，存储类型为 StorageMap，使用 Blake2_128Concat 作为哈希函数，键为 kittyId，值为 Kitty 结构体
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, kittyId, Kitty>;

	// 定义 KittyOwner 存储，使用泛型参数 T，存储类型为 StorageMap，使用 Blake2_128Concat 作为哈希函数，键为 kittyId，值为 T::AccountId 类型
	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, kittyId, T::AccountId>;

	// 定义 Event 枚举，使用泛型参数 T，包含三个事件：KittyCreated、KittyBred 和 KittyTransferred
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, kittyId, Kitty),
		KittyBred(T::AccountId, kittyId, Kitty),
		KittyTransferred(T::AccountId, T::AccountId, kittyId),
	}

	// 定义 Error 枚举，使用泛型参数 T，包含三个错误：InvalidKittyId、NotOwner 和 SameKittyId
	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		NotOwner,
		SameKittyId,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let dna = Self::random_value(&who);
			let kitty = Kitty(dna);

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + 1);

			// Emit an event.
			Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: kittyId, kitty_id_2: kittyId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check kitty id
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

			// get next id
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			// selector for breeding
			let selector = Self::random_value(&who);

			let mut data = [0u8; 16];
			for i in 0..kitty_1.0.len() {
				// 0 choose kitty2, and 1 choose kitty1
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			}
			let new_kitty = Kitty(data);

			<Kitties<T>>::insert(kitty_id, &new_kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + 1);

			Self::deposit_event(Event::KittyCreated(who, kitty_id, new_kitty));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfer(origin: OriginFor<T>, kitty_id: u32, new_owner: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);

			<KittyOwner<T>>::insert(kitty_id, new_owner);

			Ok(())
		}

	}

	impl<T: Config> Pallet<T> {
		// get a random 256.
		fn random_value(sender: &T::AccountId) -> [u8; 16] {

			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet::<T>>::extrinsic_index(),
			);

			payload.using_encoded(blake2_128)
		}

		// get netx id
		fn get_next_id() -> Result<kittyId, ()> {
			match Self::next_kitty_id() {
				kittyId::MAX => Err(()),
				val => Ok(val),
			}
		}

		// get kitty via id
		fn get_kitty(kitty_id: kittyId) -> Result<Kitty, ()> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}

	}
}
