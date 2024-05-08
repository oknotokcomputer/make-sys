#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi;

mod make_sys {
    include!(concat!(env!("OUT_DIR"), "/make/src/bindings.rs"));
}
struct Floc(make_sys::floc);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make() {
        let mut floc = Floc(make_sys::floc {
            lineno: 0,
            filenm: std::ptr::null_mut(),
            offset: 0,
        });
    }
}
