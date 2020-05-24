#![allow(unused)]
#![allow(non_camel_case_types)]
#[macro_use]
extern crate intrusive_collections;
extern crate cgc_single_threaded as cgc;

#[macro_export]
macro_rules! offset_of {
    ($ty: ty, $field: ident) => {
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize }
    };
}

#[macro_export]
macro_rules! trace_if {
    ($cond: expr, $($t: tt)*) => {
        if $cond {
            log::trace!($($t)*);
        }
    };
}

#[macro_export]
macro_rules! unwrap {
    ($e: expr) => {
        match $e {
            Ok(x) => x,
            _ => unreachable!(),
        }
    };
}
#[cfg(target_arch = "x86_64")]
macro_rules! call {
    (before ) => {};
    (after) => {};
}

pub mod common;
pub mod frontend;
pub mod runtime;

pub use runtime::get_rt;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
pub use common::rc::Rc;

#[cfg(test)]
mod tests {}
