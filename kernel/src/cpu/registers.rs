
#[derive(Debug)]
pub struct SStatus {
	value: u64
}

impl SStatus {
	pub fn new(value: u64) -> SStatus {
		SStatus {value}
	}
}