static mut BOOT_HART_ID: usize = 0;

use core::arch::{asm};

// global_asm!(include_str!("asm/boot.S"));
// global_asm!(include_str!("asm/mem.S"));


#[naked]
#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _start(hart_id: usize, fdt: usize) -> ! {
	asm!(
	"
            .option push
            .option norelax
            lla gp, __global_pointer$
            .option pop
            lla sp, __stack_start
            lla t0, __bss_start
            lla t1, __bss_end
            // Clear BSS
            1:
                beq t0, t1, 2f
                sw zero, (t0)
                addi t0, t0, 4
                j 1b
            2:
                lla t2, {boot_hart_id}
                sw a0, 0(t2)
                lla t2, {fail}
                csrw stvec, t2
                j kinit
    ",
	boot_hart_id = sym BOOT_HART_ID,
	fail = sym fail,
	options(noreturn),
	)
}

#[repr(align(4))]
extern "C" fn fail() -> ! {
	loop {}
}