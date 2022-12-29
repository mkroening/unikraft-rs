#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use $crate::io::Write;
        write!($crate::io::Stdout::new(), $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use $crate::io::Write;
        writeln!($crate::io::Stdout::new(), $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        use $crate::io::Write;
        write!($crate::io::Stderr::new(), $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {{
        use $crate::io::Write;
        write!($crate::io::Stderr::new(), $($arg)*).unwrap();
    }};
}

// Adapted from rom `std::dbg`
#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        $crate::eprintln!("[{}:{}]", ::core::file!(), ::core::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::eprintln!("[{}:{}] {} = {:#?}",
                    ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
