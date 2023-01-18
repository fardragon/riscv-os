pub mod memory_layout;
pub mod kalloc;
pub mod mmu;


pub use memory_layout::{Address, get_kernel_end, get_kernel_base, get_kernel_data_start,
						get_kernel_data_end, PAGE_SIZE, MAX_SV39_ADDRESS};
pub use kalloc::{kalloc_init, kalloc, kzalloc, kfree};
pub use mmu::mmu_init;