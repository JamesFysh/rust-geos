
#![crate_name="geos"]
#![crate_type="lib"]

extern crate libc;

use libc::{c_char, c_void};

#[link(name = "geos_c")]
extern {    
    fn initGEOS() -> *mut c_void;
    fn GEOSversion() -> *const c_char;
}

pub fn version() -> String {
    unsafe {
        std::string::raw::from_buf(GEOSversion() as *const u8)
    }
}

pub fn init() -> *mut c_void {
    unsafe {
        initGEOS()
    }
}
