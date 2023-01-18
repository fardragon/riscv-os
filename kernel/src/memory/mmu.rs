use core::mem::size_of;
use crate::memory::{Address, kzalloc, PAGE_SIZE, MAX_SV39_ADDRESS, get_kernel_base, get_kernel_data_start, get_kernel_data_end, get_kernel_end};
use crate::memory::memory_layout::get_kernel_text_end;
use crate::{kprintln_early, kprint_early, EarlyConsole, cpu};


#[repr(C)]
struct PageTableEntry {
	entry: u64,
}

#[derive(Copy, Clone)]
enum PageTableEntryField {
	Valid = 0,
	Read = 1,
	Write = 2,
	Execute = 3,

}

impl PageTableEntry {
	fn is_valid(&self) -> bool {
		self.entry & (1 << PageTableEntryField::Valid as u8) == 1
	}
	fn set_valid(&mut self) {
		self.set_permission(PageTableEntryField::Valid);
	}

	fn to_inner_page_table(&mut self) -> *mut PageTable {
		((self.entry as u64 >> 10) << 12) as *mut PageTable
	}

	fn set_from_page_table(&mut self, page_table: *const PageTable) {
		self.entry = ((page_table as u64) >> 12) << 10
	}

	fn set_from_physical_address(&mut self, physical_address: Address) {
		self.entry = (physical_address.get() >> 12) << 10
	}

	fn set_permission(&mut self, permission: PageTableEntryField) {
		self.entry |= 1 << permission as u8
	}

	fn set_permissions(&mut self, permissions: &[PageTableEntryField]) {
		for perm in permissions {
			self.set_permission(*perm);
		}
	}


}

#[repr(C)]
struct PageTable {
	entries: [PageTableEntry; 512],
}

impl PageTable {
	unsafe fn map_pages(&mut self, virtual_address: Address, physical_address: Address, size: u64, permissions: &[PageTableEntryField]) {
		if size == 0 {
			panic!("Trying to map 0 bytes!")
		}

		let mut current_page = virtual_address.round_down_to_page_boundary();
		let mut pa = physical_address;
		let last_page = virtual_address.add(size - 1).round_down_to_page_boundary();

		kprintln_early!("Mapping PA: {:p} to VA: {:p}", pa.get_ptr(), current_page.get_ptr());


		while current_page <= last_page {


			let pte = self.walk(current_page, true).unwrap();
			// kprintln_early!("Mapping PA: {:p} to VA: {:p} with PTE: {:p}", pa.get_ptr(), current_page.get_ptr(), pte);


			if (*pte).is_valid() {
				panic!("PageTable map_pages: mapping overlap!");
			}

			(*pte).set_from_physical_address(pa);
			(*pte).set_valid();
			(*pte).set_permissions(permissions);



			current_page = current_page.add(PAGE_SIZE);
			pa = pa.add(PAGE_SIZE);
		}
	}

	fn vpn(virtual_address: Address, level: u8) -> usize {
		match level {
			2 => ((virtual_address.get() >> 30) & 0x1FF) as usize,
			1 => ((virtual_address.get() >> 21) & 0x1FF) as usize,
			0 => ((virtual_address.get() >> 12) & 0x1FF) as usize,
			_ => panic!("PageTable vpn: invalid level")
		}
	}

	fn get_entry(&mut self, virtual_address: Address, level: u8) -> Result<*mut PageTableEntry, &'static str> {
		Ok(unsafe { self.entries.as_mut_ptr().add(PageTable::vpn(virtual_address, level)) })
	}

	unsafe fn walk(&mut self, virtual_address: Address, allocate_missing: bool) -> Result<*mut PageTableEntry, &'static str> {
		// if virtual_address >= MAX_SV39_ADDRESS {
		// 	return Err("Virtual address overflow");
		// }


		let mut pagetable: *mut PageTable = self;

		for level in [2, 1] {
			let pte = (*pagetable).get_entry(virtual_address, level).unwrap();
			if (*pte).is_valid() {
				pagetable = (*pte).to_inner_page_table();
			} else {
				if allocate_missing {
					pagetable = kzalloc() as *mut PageTable;
					(*pte).set_from_page_table(pagetable);
					(*pte).set_valid()
				} else {
					return Err("PageTableEntry not found!");
				}
			}
		}
		Ok((*pagetable).entries.as_mut_ptr().add(PageTable::vpn(virtual_address, 0)))
	}
}


pub fn mmu_init() {
	assert_eq!(size_of::<PageTable>(), PAGE_SIZE as usize, "Invalid PageTable size");
	assert_eq!(size_of::<PageTableEntry>(), 8 as usize, "Invalid PageTableEntry size");

	let mut kernel_page_table = kzalloc() as *mut PageTable;

	kprintln_early!("Allocated kernel page table at: {:p}", kernel_page_table);


	unsafe {
		// Map kernel code: R+X
		kprintln_early!("Mapping kernel text");
		(*kernel_page_table).map_pages(get_kernel_base(), get_kernel_base(), get_kernel_text_end() - get_kernel_base(), &[PageTableEntryField::Read, PageTableEntryField::Execute]);

		// Map kernel memory: R+W
		kprintln_early!("Mapping kernel data and stack");
		(*kernel_page_table).map_pages(get_kernel_data_start(), get_kernel_data_start(), get_kernel_end() - get_kernel_data_start(), &[PageTableEntryField::Read, PageTableEntryField::Write]);
	}

	cpu::RISCV64{}.sfence_vma();
	let satp: u64 = ((8 as u64) << 60) | ((kernel_page_table as u64) >> 12);
	cpu::RISCV64{}.write_satp(Address(satp));
	cpu::RISCV64{}.sfence_vma();

	kprintln_early!("MMU init complete!");
}