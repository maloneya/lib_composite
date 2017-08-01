// We kinda need to run on nightly to make xargo work
// However, it would be nice to avoid having to depend on unstable features
// TODO: Investigate the stabilization of "shared"
#![feature(shared)]
// TODO: Investigate switching to marker types
#![feature(optin_builtin_traits)]
// TODO: Remove uses of FnBox once Box<FnOnce()> just works
#![feature(fnbox)]

extern crate libc;

extern crate libc_extra;

// We put this before "mod" definitions, so they use this print macro
macro_rules! print {
    ($($arg:tt)*) => ($crate::print_impl::print_args(format_args!($($arg)*)));
}

mod print_impl;
mod sys;

pub mod kernel_api;
pub mod sl;
pub mod sl_lock;

use kernel_api::DefKernelAPI;
use sl::Sl;


#[no_mangle]
pub extern "C" fn rust_init() {
    let def_api = DefKernelAPI::from_standard_boot_capabilities();

    println!("This is a print from rust!");


    Sl::start_scheduler_loop(def_api, 30, move |sl| {
        println!("This is a print from the new thread!");
    });
}
