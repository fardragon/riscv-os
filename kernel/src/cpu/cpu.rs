use core::arch::asm;
use crate::cpu::registers::SStatus;

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
}