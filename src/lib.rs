
#![crate_name="geos"]
#![crate_type="lib"]

extern crate libc;

use libc::{c_char};

#[link(name = "geos_c")]
extern {    
    fn GEOSversion() -> *const c_char;
}

pub fn version() -> String {
    unsafe {
        std::string::raw::from_buf(GEOSversion() as *const u8)
    }
}

