#![cfg_attr(target_os = "linux", feature(c_unwind))]
#![cfg_attr(target_os = "linux", feature(lang_items))]
#![no_std]
#![no_main]

unikraft::can_run_this!();

extern crate alloc;

use alloc::string::String;
use core::ffi::{c_char, c_int};

use unikraft_abi::println;

#[no_mangle]
extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut message = String::from("Hello, ");
    message.push_str("allocations!");
    println!("{message}");

    0
}

#[cfg(target_os = "linux")]
#[lang = "eh_personality"]
fn eh_personality() {}

#[cfg(target_os = "linux")]
#[no_mangle]
pub extern "C-unwind" fn _Unwind_Resume(_exception: *mut ()) -> ! {
    unreachable!()
}
