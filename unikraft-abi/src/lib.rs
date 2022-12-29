#![no_std]
#![warn(rust_2018_idioms)]

use core::ffi::{c_char, c_int};

pub type Main = extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int;

pub mod alloc;

pub mod guest;

// Migrate to core::io once available:
// https://github.com/rust-lang-nursery/portability-wg/issues/12
pub mod io;

mod macros;

#[cfg(feature = "panic_handler")]
mod panicking;
