#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::sp_runtime::RuntimeDebug;
pub use pallet::*;
use scale_info::TypeInfo;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use codec::{Decode, Encode};
	use frame_support::{pallet, pallet_prelude::*, traits::StoredMap, BoundedVec};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// type Todo: Parameter + Member + Default + Copy + PartialEq + Encode + Decode +
		// MaxEncodedLen; type TodoStorage: Parameter
		// 	+ Member
		// 	+ Default
		// 	+ Copy
		// 	+ PartialEq
		// 	+ Encode
		// 	+ Decode
		// 	+ MaxEncodedLen;

		// type TodoList: Parameter
		// 	+ Member
		// 	+ Default
		// 	+ Copy
		// 	+ PartialEq
		// 	+ Encode
		// 	+ Decode
		// 	+ MaxEncodedLen;
		type Todo2: Get<Todo>;

		type TodoList2: StoredMap<Self::AccountId, BoundedVec<Todo, ConstU32<100>>>;

		type TodoCount: Parameter
			+ Member
			+ Default
			+ Copy
			+ PartialEq
			+ Encode
			+ Decode
			+ MaxEncodedLen;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Todo not found
		TodoNotFound,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Todo {
		pub title: [u8; 128],
		pub description: [u8; 256],
		pub done: bool,
		pub created_at: u64,
	}

	impl Default for Todo {
		fn default() -> Self {
			Todo { title: [0; 128], description: [0; 256], done: false, created_at: 0 }
		}
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct TodoList(BoundedVec<Todo, ConstU32<100>>);

	#[pallet::storage]
	pub type TodoMap<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::TodoList2, ValueQuery>;

	#[pallet::storage]
	pub type TodoCount<T: Config> = StorageValue<_, u32>;

	impl<T: Config> Pallet<T> {
		pub fn get_todos_by_account_id(
			account_id: &T::AccountId,
		) -> Result<T::TodoList2, Error<T>> {
			let todos = TodoMap::<T>::try_get(account_id).map_err(|_| Error::<T>::TodoNotFound)?;
			Ok(todos)
		}

		pub fn insert_todo_by_account_id(
			account_id: &T::AccountId,
			todo: T::Todo2,
		) -> Result<(), Error<T>> {
			let mut todos = Self::get_todos_by_account_id(account_id).unwrap_or_default();
			todos.0.push(todo.into());
			TodoMap::<T>::insert(account_id, todos);
			Ok(())
		}

		pub fn get_todo_count() -> u32 {
			TodoCount::<T>::get().expect("failed to get todo count")
		}

		pub fn increment_todo_count() {
			let count = Self::get_todo_count();
			TodoCount::<T>::put(count + 1);
		}
	}
}
