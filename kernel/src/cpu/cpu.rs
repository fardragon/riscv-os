use core::arch::asm;
use crate::cpu::registers::{SStatus, SCause};
use crate::memory::Address;

pub struct RISCV64 {

}

impl RISCV64 {
	pub fn sstatus(&self) -> SStatus {
		let mut value: u64;
		unsafe {
			asm!(
			"
				csrrs {value}, sstatus, x0
			",
			value = out(reg) value,
			);
		}

		SStatus::new(value)
	}

	pub fn scause(&self) -> SCause {
		let mut value: u64;
		unsafe {
			asm!(
			"
				csrrs {value}, scause, x0
			",
			value = out(reg) value,
			);
		}

		SCause::new(value)
	}

	pub fn sfence_vma(&self) {
		unsafe {
			asm!(
			"
				sfence.vma zero, zero
			",
			);
		}
	}

	pub fn write_satp(&mut self, address: Address) {
		unsafe {
			asm!(
			"
				csrw satp, {value}
			",
			value = in(reg) address.get(),
			);
		}
	}
}