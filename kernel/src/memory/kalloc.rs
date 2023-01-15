use crate::memory::{get_kernel_end, get_page_size};
use crate::sync::Mutex;

#[repr(C)]
#[repr(align(4096))]
struct Page {
	next: *mut Page,
}

const PAGE_SIZE: u64 = get_page_size();
const ALLOC_PAGE_SENTINEL: u8 = 5;
const DEALLOC_PAGE_SENTINEL: u8 = 1;

struct KernelPageAllocator {
	free_list: *mut Page
}

impl KernelPageAllocator {
	const fn new() -> KernelPageAllocator {
		KernelPageAllocator {
			free_list: 0 as *mut Page
		}
	}

	fn init(&mut self) {
		let start = unsafe{(KernelPageAllocator::align_to_page_size(get_kernel_end()))};
		let end = 128 * 1024 * 1024;

		assert_eq!(start as u64 % PAGE_SIZE, 0, "Unaligned first page")

	}

	fn deallocate_page(&mut self, memory: *mut u8) {
		unsafe {
			KernelPageAllocator::memset_page(memory, DEALLOC_PAGE_SENTINEL);
			let page = memory as *mut Page;
			(*page).next = self.free_list;
			self.free_list = page;
		}
	}

	fn allocate_page(&mut self) -> *mut u8 {

		if self.free_list == 0 as *mut Page {
			panic!("Failed to allocate page");
		}


		unsafe {
			let page = self.free_list;
			self.free_list = (*page).next;

			KernelPageAllocator::memset_page(page as *mut u8, ALLOC_PAGE_SENTINEL);

			page as *mut u8
		}
	}

	unsafe fn align_to_page_size(pointer : *mut u8) -> *mut u8 {
		let offset = PAGE_SIZE as usize - (pointer as u64 % PAGE_SIZE) as usize;
		pointer.add(offset)
	}

	unsafe fn memset_page(memory: *mut u8, value: u8) {
		for offset in 0..PAGE_SIZE as usize {
			memory.add(offset).write(value);
		}
	}
}


static ALLOCATOR: Mutex<KernelPageAllocator> = Mutex::new(KernelPageAllocator::new(), "KernelPageAllocator");

pub fn kalloc_init() {
	ALLOCATOR.lock().init();
}

pub fn kalloc() -> *mut u8 {
	ALLOCATOR.lock().allocate_page()
}

pub fn kfree(pointer: *mut u8) {
	ALLOCATOR.lock().deallocate_page(pointer)
}