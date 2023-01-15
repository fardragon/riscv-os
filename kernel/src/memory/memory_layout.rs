

extern "C" {
	static __kernel_end: u64;
}

pub fn get_kernel_end() -> *mut u8 {
	unsafe{&__kernel_end as *const u64 as *mut u8}
}

pub const fn get_page_size() -> u64 {
	4096
}