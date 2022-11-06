#![no_std]
#![warn(rust_2018_idioms)]

#[macro_export]
macro_rules! can_run_this {
    () => {};
}

#[cfg(feature = "kvm")]
macro_rules! entry_symbol {
    () => {
        "_libkvmplat_entry"
    };
}

#[cfg(feature = "linuxu")]
macro_rules! entry_symbol {
    () => {
        "_liblinuxuplat_start"
    };
}

#[cfg(any(feature = "kvm", feature = "linuxu"))]
core::arch::global_asm!(
    ".global _unikraft_rs_start",
    "_unikraft_rs_start:",
    concat!("jmp ", entry_symbol!()),
);
