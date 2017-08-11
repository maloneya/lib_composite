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

//// We put this before "mod" definitions, so they use this print macro
//#[macro_export]
//macro_rules! print {
//    ($($arg:tt)*) => ($crate::print_impl::print_args(format_args!($($arg)*)));
//}

pub mod sys;

pub mod kernel_api;
pub mod sl;
pub mod sl_lock;