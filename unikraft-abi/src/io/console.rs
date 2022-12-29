use core::ffi::c_int;

use crate::io::{Read, Result, Write};

/// Reads from the kernel console.
pub struct Stdin(());

impl Stdin {
    pub const fn new() -> Self {
        Self(())
    }
}

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len().min(usize::try_from(c_int::MAX).unwrap());
        let buf = &mut buf[..len];
        let ret = unsafe { raw::ukplat_cink(buf.as_mut_ptr().cast(), len.try_into().unwrap()) };
        super::try_return_code(ret)
    }
}

/// Writes to the kernel console.
pub struct Stdout(());

impl Stdout {
    pub const fn new() -> Self {
        Self(())
    }
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len().min(usize::try_from(c_int::MAX).unwrap());
        let buf = &buf[..len];
        let ret = unsafe { raw::ukplat_coutk(buf.as_ptr().cast(), len.try_into().unwrap()) };
        super::try_return_code(ret)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
/// Writes to the debug console.
pub struct Stderr(());

impl Stderr {
    pub const fn new() -> Self {
        Self(())
    }
}

impl Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len().min(usize::try_from(c_int::MAX).unwrap());
        let buf = &buf[..len];
        let ret = unsafe { raw::ukplat_coutd(buf.as_ptr().cast(), len.try_into().unwrap()) };
        super::try_return_code(ret)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

/// From `unikraft/include/uk/plat/console.h`
mod raw {
    use core::ffi::{c_char, c_int, c_uint};

    extern "C" {
        /// Outputs a string to kernel console
        /// Note that printing does not stop on null termination
        /// @param buf Buffer with characters
        /// @param len Length of string buffer (if 0 str has to be zero-terminated),
        ///            < 0 ignored
        /// @return Number of printed characters, errno on < 0
        pub fn ukplat_coutk(buf: *const c_char, len: c_uint) -> c_int;

        /// Outputs a string to debug console
        /// Note that printing does not stop on null termination
        /// @param buf Buffer with characters
        /// @param len Length of string buffer (if 0 str has to be zero-terminated)
        /// @return Number of printed characters, errno on < 0
        pub fn ukplat_coutd(buf: *const c_char, len: c_uint) -> c_int;

        /// Reads characters from kernel console
        /// Note that returned buf is not null terminated.
        /// @param buf Target buffer
        /// @param len Length of string buffer
        /// @return Number of read characters, errno on < 0
        pub fn ukplat_cink(buf: *mut c_char, maxlen: c_uint) -> c_int;
    }
}
