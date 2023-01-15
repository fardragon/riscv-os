use core::fmt::{Error, Write};
use sbi;
use sbi::legacy;

pub struct EarlyConsole {
}

impl Write for EarlyConsole {
	fn write_str(&mut self, out: &str) -> Result<(), Error> {
		for c in out.bytes() {
			legacy::console_putchar(c);
		}
		Ok(())
	}
}



#[macro_export]
macro_rules! kprint_early
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(EarlyConsole{}, $($args)+);
			});
}
#[macro_export]
macro_rules! kprintln_early
{
	() => ({
		   kprint_early!("\r\n")
		   });
	($fmt:expr) => ({
			kprint_early!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			kprint_early!(concat!($fmt, "\r\n"), $($args)+)
			});
}