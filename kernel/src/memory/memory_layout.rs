use core::ops::Sub;

extern "C" {
	static __kernel_end: u64;
	static __text_end: u64;
	static __data_start: u64;
	static __data_end: u64;
}

pub const PAGE_SIZE: u64 = 4096 as u64;
pub const MAX_SV39_ADDRESS: Address = Address(1 << (9 + 9 + 9 + 12 - 1));

#[derive(Clone, Copy, PartialOrd, PartialEq)]
pub struct Address(pub u64);


impl Address {
	pub fn get(&self) -> u64 {
		self.0
	}

	pub fn get_ptr(&self) -> *const u8 {
		self.0 as *const u8
	}

	pub fn get_mut_ptr(&self) -> *mut u8 {
		self.0 as *mut u8
	}

	pub fn add(&self, offset: u64) -> Self {
		Address(self.0 + offset)
	}

	pub fn round_up_to_page_boundary(&self) -> Address {
		let offset = PAGE_SIZE - self.0  % PAGE_SIZE;
		Address(self.0 + offset)
	}

	pub fn round_down_to_page_boundary(&self) -> Address {
		let offset = self.0  % PAGE_SIZE;
		Address(self.0 - offset)
	}
}

impl Sub for Address {
	type Output = u64;
	fn sub(self, rhs: Self) -> Self::Output {
		self.0 - rhs.0
	}
}


pub fn get_kernel_end() -> Address {
	Address(unsafe { &__kernel_end } as *const _ as u64)
}

pub fn get_kernel_base() -> Address {
	Address(0x80000000)
}

pub fn get_kernel_text_end() -> Address {
	Address(unsafe { &__text_end } as *const _ as u64)
}
pub fn get_kernel_data_start() -> Address {
	Address(unsafe { &__data_start } as *const _ as u64)
}
pub fn get_kernel_data_end() -> Address {
	Address(unsafe { &__data_end } as *const _ as u64)
}

