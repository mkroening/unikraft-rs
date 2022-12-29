use core::panic::PanicInfo;

use crate::guest::{self, ExitMode};
use crate::io::{Stderr, Write};

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    writeln!(Stderr::new(), "{info}").ok();
    guest::exit(ExitMode::Crash)
}
