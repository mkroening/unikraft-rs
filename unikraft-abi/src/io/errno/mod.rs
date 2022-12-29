//! This module defines an [`Error`] struct that represents error numbers
//! returned from Unikraft system calls.
// based on the `linux-errno` crate

use core::fmt;
use core::num::NonZeroU16;

/// Type for error numbers returned from Unikraft system calls.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
pub struct Error(NonZeroU16);

impl Error {
    /// Create a new error from a raw error number. If outside the permitted
    /// range `[1, 4096)` for Unikraft error numbers, returns `None`.
    pub const fn new(errno: u16) -> Option<Error> {
        if errno > 0xFFF {
            return None;
        }
        match NonZeroU16::new(errno) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }

    /// Unsafely create a new error from a raw error number, without checking
    /// whether it's within the permitted range for Unikraft error numbers.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `0 < errno <= 0xFFF`. In particular, it is
    /// undefined behavior if `errno` is zero.
    pub const unsafe fn new_unchecked(errno: u16) -> Error {
        Error(NonZeroU16::new_unchecked(errno))
    }

    /// Returns the error number as a primitive `u16`.
    pub const fn get(&self) -> u16 {
        self.0.get()
    }

    /// Returns the error number as a [`NonZeroU16`].
    pub const fn get_nonzero(&self) -> NonZeroU16 {
        self.0
    }
}

impl From<Error> for u16 {
    fn from(err: Error) -> u16 {
        err.0.get()
    }
}

impl From<Error> for NonZeroU16 {
    fn from(err: Error) -> NonZeroU16 {
        err.0
    }
}

impl From<Error> for i32 {
    fn from(err: Error) -> i32 {
        err.0.get().into()
    }
}

/// The error type returned when a checked integral type conversion fails.
#[derive(Debug)]
pub struct TryFromIntError;

impl fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("raw error number out of range")
    }
}

impl TryFrom<u16> for Error {
    type Error = TryFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(TryFromIntError)
    }
}

impl TryFrom<i32> for Error {
    type Error = TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        u16::try_from(value)
            .map_err(|_err| TryFromIntError)
            .and_then(|value| Self::new(value).ok_or(TryFromIntError))
    }
}

impl PartialEq<i32> for Error {
    fn eq(&self, other: &i32) -> bool {
        i32::from(self.get()).eq(other)
    }
}

impl PartialEq<Error> for i32 {
    fn eq(&self, other: &Error) -> bool {
        self.eq(&i32::from(other.get()))
    }
}

macro_rules! errno_constants {
	( $( $(#[$meta:meta])* $name:ident = $value:literal , )+ ) => {
		$(
			$(#[$meta])*
			pub const $name: super::Error = {
				match super::Error::new($value) {
                    Some(error) => error,
                    None => unreachable!(),
                }
			};
		)*

		pub(crate) const fn err_name(err: super::Error) -> Option<&'static str> {
			match err.0.get() {
			$(
				$value => Some(stringify!($name)),
			)*
				_ => None,
			}
		}
	}
}

mod constants;

pub use constants::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match constants::err_name(*self) {
            Some(name) => f.write_str(name),
            _ => f.debug_tuple("Error").field(&self.0.get()).finish(),
        }
    }
}
