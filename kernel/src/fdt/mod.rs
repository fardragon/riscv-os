use crate::{kprintln_early, kprint_early};
use crate::io::early_console::EarlyConsole;

#[derive(Debug)]
struct FDTHeader {
	address: *const u32,
	total_size: u32,
	off_dt_struct: u32,
	off_dt_strings: u32,
	off_mem_rsvmap: u32,
	version: u32,
	last_comp_version: u32,
	boot_cpuid_phys: u32,
	size_dt_strings: u32,
	size_dt_struct: u32,
}

#[derive(Debug)]
struct FDTReserveEntry {
	address: u64,
	size: u64
}

impl FDTReserveEntry {
	pub fn new(address: *const u64) -> FDTReserveEntry {
		FDTReserveEntry {
			address: unsafe{address.add(0).read()}.swap_bytes(),
			size: unsafe{address.add(1).read()}.swap_bytes(),
		}
	}
}


const FDT_MAGIC: u32 = (0xd00dfeed as u32).to_be();

#[derive(Debug)]
pub struct DeviceTree {
	header: FDTHeader
}

impl DeviceTree {
	pub fn new(address: *const u32) -> Result<DeviceTree, &'static str> {


	let magic = unsafe{address.read()};

	if magic != FDT_MAGIC {
		return Err("Invalid magic number");
	}

		Ok(DeviceTree {
			header: FDTHeader {
				address,
				total_size: unsafe{address.add(1).read()}.swap_bytes(),
				off_dt_struct: unsafe{address.add(2).read()}.swap_bytes(),
				off_dt_strings: unsafe{address.add(3).read()}.swap_bytes(),
				off_mem_rsvmap: unsafe{address.add(4).read()}.swap_bytes(),
				version: unsafe{address.add(5).read()}.swap_bytes(),
				last_comp_version: unsafe{address.add(6).read()}.swap_bytes(),
				boot_cpuid_phys: unsafe{address.add(7).read()}.swap_bytes(),
				size_dt_strings: unsafe{address.add(8).read()}.swap_bytes(),
				size_dt_struct:unsafe{address.add(9).read()}.swap_bytes(),
			}
		})
	}


	pub fn walk(&self) -> () {

		// Memory reservation map
		{
			let mut it = self.memory_reservation_head();
			loop {
				let entry = FDTReserveEntry::new(it);

				match entry {
					FDTReserveEntry{address: 0, size: 0} => break,
					FDTReserveEntry{address, size} => {
						kprintln_early!("Reserved memory entry at {:x} size {}", address, size)
					}
				}
				it = unsafe{it.add(2)}
			}
		}

		// Structure block
		{

		}


	}

	fn memory_reservation_head(&self) -> *const u64 {
		unsafe{(self.header.address as *const u64).add((self.header.off_mem_rsvmap / core::mem::size_of::<u64>() as u32) as usize)}
	}

}
