use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct Mutex<T> {
	name: &'static  str,
	locked: AtomicBool,
	data: UnsafeCell<T>,
}

#[derive(Debug)]
pub struct MutexGuard<'a, T: 'a> {
	mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
	pub const fn new(value: T, name: &'static str) -> Mutex<T> {
		Mutex {
			name,
			locked: AtomicBool::new(false),
			data: UnsafeCell::new(value)
		}
	}

	pub fn lock(&self) -> MutexGuard<'_, T> {
		loop {
			if !self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
				break MutexGuard {
					mutex: self
				}
			}
			core::hint::spin_loop();
		}
	}

	pub fn unlock(guard: MutexGuard<'_, T>) -> &'_ Mutex<T> {
		guard.mutex()
	}
}

unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}

impl<'a, T: 'a> MutexGuard<'a, T> {
	// Returns a reference to the original 'Mutex' object.
	pub fn mutex(&self) -> &'a Mutex<T> {
		self.mutex
	}
}

impl<'a, T: 'a> Drop for MutexGuard<'a, T> {
	fn drop(&mut self) {
		// assert!(self.holding(), "release {}", self.mutex.name);
		self.mutex.locked.store(false, Ordering::Release);
	}
}

impl<'a, T: 'a> Deref for MutexGuard<'a, T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.mutex.data.get() }
	}
}

impl<'a, T: 'a> DerefMut for MutexGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.mutex.data.get() }
	}
}