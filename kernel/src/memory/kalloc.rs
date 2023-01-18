use core::mem;
use crate::{kprintln_early, kprint_early, EarlyConsole};
use crate::memory::{get_kernel_end, PAGE_SIZE, Address};
use crate::sync::Mutex;

#[repr(C)]
#[repr(align(4096))]
struct Page {
	next: *mut Page,
}

const ALLOC_PAGE_SENTINEL: u8 = 5;
const DEALLOC_PAGE_SENTINEL: u8 = 1;

struct KernelPageAllocator {
	free_list: *mut Page,
}

impl KernelPageAllocator {
	const fn new() -> KernelPageAllocator {
		KernelPageAllocator {
			free_list: 0 as *mut Page
		}
	}

	fn init(&mut self) {
		let mut start = get_kernel_end().round_up_to_page_boundary();
		let end = Address(0x80000000).add(128 * 1024 * 1024);

		assert_eq!(start.get() % PAGE_SIZE, 0, "Unaligned first page");

		kprintln_early!("KernelPageAllocator init start: {:p} end: {:p}", start.get_ptr(), end.get_ptr());

		while start < end {
			self.deallocate_page(start);
			start = start.add(PAGE_SIZE);
		}
	}

	fn deallocate_page(&mut self, memory: Address) {
		unsafe {
			// KernelPageAllocator::memset_page(memory, DEALLOC_PAGE_SENTINEL);

			let page = memory.get_mut_ptr() as *mut Page;
			(*page).next = self.free_list;
			self.free_list = page;
		}
	}

	fn allocate_page(&mut self, sentinel: u8) -> Address {
		if self.free_list == 0 as *mut Page {
			panic!("Failed to allocate page");
		}


		unsafe {
			let page = self.free_list;
			self.free_list = (*page).next;

			KernelPageAllocator::memset_page(Address(page as u64), sentinel);
			Address(page as u64)
		}
	}



	unsafe fn memset_page(memory: Address, value: u8) {
		let c: usize = mem::transmute([value as u8; 8]);
		let n_usize: u64 = PAGE_SIZE / 8;
		let mut i: u64 = 0;

		// Set `WORD_SIZE` bytes at a time
		let n_fast = n_usize * 8;
		while i < n_fast {
			*((memory.get() + i) as *mut usize) = c;
			i += 8;
		}

		let c = c as u8;

		// Set 1 byte at a time
		while i < PAGE_SIZE {
			*((memory.get() + i) as *mut u8) = c;
			i += 1;
		}
	}
}


static ALLOCATOR: Mutex<KernelPageAllocator> = Mutex::new(KernelPageAllocator::new(), "KernelPageAllocator");

pub fn kalloc_init() {
	ALLOCATOR.lock().init();
}

pub fn kalloc() -> *mut u8 {
	ALLOCATOR.lock().allocate_page(ALLOC_PAGE_SENTINEL).get_mut_ptr()
}

pub fn kzalloc() -> *mut u8 {
	ALLOCATOR.lock().allocate_page(0).get_mut_ptr()
}

pub fn kfree(pointer: *mut u8) {
	ALLOCATOR.lock().deallocate_page(Address(pointer as u64))
}