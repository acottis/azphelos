
use core::fmt::{Result,Write};

pub struct ScreenWriter;

impl Write for ScreenWriter {
    fn write_str(&mut self, string: &str) -> Result {
        crate::efi::output_string(string);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        <$crate::print::ScreenWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::ScreenWriter,
            core::format_args!($($arg)*)).unwrap();
    }        
}