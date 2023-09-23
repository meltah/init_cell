//! A cell which can be unsafely initialized or interiorly mutated, but safely accessed.
//! 
//! This is mostly intended for use in statics. The cell is safe to access, but must be initialized before any access. There is no synchronization to ensure initialization is observed, so you should initialize at the beginning of the main function or using something like the `ctor` crate.
//! 
//! # Example
//! 
//! ```
//! use init_cell::InitCell;
//! 
//! // SAFETY: We will initialize the cell before using it.
//! pub static MY_VAL: InitCell<Vec<i32>> = unsafe { InitCell::new() };
//! 
//! fn main() {
//! 	// SAFETY: Nothing is accessing the cell.
//! 	unsafe {
//! 		InitCell::init(&MY_VAL, vec![1, 2, 3]);
//! 	}
//! 	assert_eq!(MY_VAL.iter().sum::<i32>(), 6);
//! 
//! 	// The cell can be mutated, too, which drops the previous value.
//! 	unsafe {
//! 		InitCell::set(&MY_VAL, vec![4, 5, 6]);
//! 	}
//! 	assert_eq!(MY_VAL.iter().sum::<i32>(), 15);
//! }
//! ```

#![no_std]

use core::fmt;
use core::ops::{Deref, DerefMut};
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::cmp::Ordering;

/// A one-time initialization cell.
/// 
/// This is mostly intended for use in statics. The cell is safe to access,
/// but must be initialized before any access. There is no synchronization
/// to ensure initialization is observed, so you should initialize at the
/// beginning of the main function or using something like the `ctor` crate.
#[repr(transparent)]
pub struct InitCell<T>(UnsafeCell<MaybeUninit<T>>);

unsafe impl<T: Send> Send for InitCell<T> {}
unsafe impl<T: Sync> Sync for InitCell<T> {}

impl<T> From<T> for InitCell<T> {
	fn from(x: T) -> Self { Self::initialized(x) }
}

impl<T: PartialEq> PartialEq for InitCell<T> {
	fn eq(&self, rhs: &Self) -> bool { **self == **rhs }
}

impl<T: Eq> Eq for InitCell<T> {}

impl<T: PartialOrd> PartialOrd for InitCell<T> {
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> { (**self).partial_cmp(&**rhs) }
}

impl<T: Ord> Ord for InitCell<T> {
	fn cmp(&self, rhs: &Self) -> Ordering { (**self).cmp(&**rhs) }
}

impl<T: fmt::Debug> fmt::Debug for InitCell<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T: fmt::Display> fmt::Display for InitCell<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T: core::hash::Hash> core::hash::Hash for InitCell<T> {
	fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
		(**self).hash(h);
	}
}

impl<T> Deref for InitCell<T> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe { (*self.0.get()).assume_init_ref() }
	}
}

impl<T> DerefMut for InitCell<T> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe { (*self.0.get()).assume_init_mut() }
	}
}

impl<T> InitCell<T> {
	/// Creates a new uninitialized `InitCell`.
	/// 
	/// # Safety
	/// The cell must be initialized before it is accessed.
	pub const unsafe fn new() -> Self { Self(UnsafeCell::new(MaybeUninit::uninit())) }
	
	/// Creates a new initialized `InitCell`. Unlike `InitCell::new`, this is
	/// safe because the cell is already initialized and can be used freely.
	pub fn initialized(x: T) -> Self { Self(UnsafeCell::new(MaybeUninit::new(x))) }

	/// Gets the inner (initialized) value of this cell.
	pub unsafe fn into_inner(cell: Self) -> T { cell.0.into_inner().assume_init() }

	/// Initializes the cell.
	/// 
	/// # Safety
	/// This must be done when there are no references to the contents of this
	/// cell, including no other threads accessing it.
	pub unsafe fn init(cell: &Self, x: T) {
		(*cell.0.get()).write(x);
	}

	/// Sets the cell's value.
	/// 
	/// # Safety
	/// This must be done when there are no references to the contents of this
	/// cell, including no other threads accessing it. Additionally, the cell
	/// must have been previously initialized, as this will drop the old value.
	pub unsafe fn set(cell: &Self, x: T) {
		*(*cell.0.get()).as_mut_ptr() = x;
	}
}
