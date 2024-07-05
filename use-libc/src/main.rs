extern crate libc;

use std::ffi::CString;

extern "C" {
    fn printf(format: *const u8, ...) -> i32;
}

fn main() {
    let cstr = CString::new("Hello, world!\n").unwrap();
    unsafe {
	printf(cstr.as_ptr() as *const u8);
    }
}
