#![feature(naked_functions, fn_align, panic_info_message)]
#![no_main]
#![no_std]


mod boot;

#[macro_use]
mod io;
use io::EarlyConsole;

mod cpu;
mod fdt;
mod memory;
mod sync;

use core::arch::asm;
use crate::fdt::DeviceTree;
use crate::memory::kalloc_init;
use crate::memory::mmu_init;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	kprint_early!("Aborting: ");
	if let Some(p) = info.location() {
		kprintln_early!(
			"line {}, file {}: {}",
			p.line(),
			p.file(),
			info.message().unwrap()
		);
	}
	else {
		kprintln_early!("no information available.");
	}
	abort();
}

#[no_mangle]
extern "C" fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi");
		}
	}
}

#[no_mangle]
extern "C" fn kinit(_hart_id: usize, fdt: *const u32) {
	kprintln_early!("\r\n\r\n\r\n");
	kprintln_early!("Booting RISC-V kernel");
	kprintln_early!(" ______");
	kprintln_early!("| |__| |");
	kprintln_early!("|  ()  |");
	kprintln_early!("|______|");

	kprintln_early!("END: {:p}", memory::get_kernel_end().get_ptr());


	kalloc_init();
	mmu_init();

	let cpu = cpu::RISCV64{};
	// kprintln_early!("{:x?}", cpu.sstatus());

	let device_tree = DeviceTree::new(fdt).expect("Failed to parse device tree");
	kprintln_early!("Device tree header: {:?}", device_tree);
	// device_tree.walk();
	kprintln_early!("Kernel init complete");

	loop {
	}
}
