mod console;
pub mod errno;

use core::ffi::c_int;
use core::{fmt, result};

pub use console::{Stderr, Stdin, Stdout};
pub use errno::Error;

pub type Result<T> = result::Result<T, Error>;

fn try_return_code(ret: c_int) -> Result<usize> {
    let abs = ret.unsigned_abs();
    if ret.is_negative() {
        Err(Error::new(abs.try_into().unwrap()).unwrap())
    } else {
        Ok(abs.try_into().unwrap())
    }
}

// Adapted from `std::io::Read`.
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

// Adapted from `std::io::Write`.
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(errno::EIO);
                }
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<()> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adapter<'a, T: ?Sized> {
            inner: &'a mut T,
            error: Result<()>,
        }

        impl<T: Write + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adapter {
            inner: self,
            error: Ok(()),
        };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `Write` or not
                if output.error.is_err() {
                    output.error
                } else {
                    Err(errno::EINVAL)
                }
            }
        }
    }
}
