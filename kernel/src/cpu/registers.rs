
#[derive(Debug)]
pub struct SStatus {
	value: u64
}

impl SStatus {
	pub fn new(value: u64) -> SStatus {
		SStatus {value}
	}
}

#[derive(Debug)]
pub struct SCause {
	interrupt: bool,
	value: u64
}

impl SCause {
	pub fn new(value: u64) -> SCause {
		SCause {
			interrupt: ((value >> 63) & 0b1) == 1,
			value: value & 0x7FFFFFFFFFFFFFFF,
		}
	}
}