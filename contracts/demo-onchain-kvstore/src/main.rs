#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(all(target_arch = "riscv64", not(test)), no_main)]

#[cfg(test)]
extern crate alloc;

#[cfg(all(target_arch = "riscv64", not(test)))]
use ckb_std::default_alloc;
#[cfg(all(target_arch = "riscv64", not(test)))]
ckb_std::entry!(program_entry);
#[cfg(all(target_arch = "riscv64", not(test)))]
default_alloc!();

// TODO ref: issue nervosnetwork/ckb-std#84
#[cfg(target_arch = "riscv64")]
macro_rules! debug {
    ($fmt:literal $(,$args:expr)* $(,)?) => {
        #[cfg(debug_assertions)]
        ckb_std::syscalls::debug(alloc::format!($fmt $(,$args)*));
    };
}

#[cfg(target_arch = "riscv64")]
mod entry;
#[cfg(target_arch = "riscv64")]
mod error;
#[cfg(target_arch = "riscv64")]
mod operations;

#[cfg(target_arch = "riscv64")]
pub fn program_entry() -> i8 {
    match entry::main() {
        Ok(_) => 0,
        Err(err) => err.into(),
    }
}

#[cfg(not(target_arch = "riscv64"))]
pub fn main() {}
