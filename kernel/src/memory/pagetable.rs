

#[repr(C)]
pub struct PagetableEntry {
	entry: u64
}

#[derive(Copy, Clone)]
enum PagetableEntryField {
	Valid = 0
}

#[repr(C)]
pub struct Pagetable {
	pub entries: [PagetableEntry; 512]
}

impl PagetableEntry {
	pub fn is_valid(&self) -> bool {
		self.entry & (1 << PagetableEntryField::Valid as u8) == 1
	}
}