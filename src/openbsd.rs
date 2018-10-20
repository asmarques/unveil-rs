use std::ffi::CString;
use std::io;

mod ffi {
    use std::os::raw::{c_char, c_int};

    extern "C" {
        pub fn unveil(path: *const c_char, permissions: *const c_char) -> c_int;
    }
}

pub fn unveil(path: &str, permissions: &str) -> Result<(), i32> {
    let cpath = CString::new(path).unwrap();
    let cpermissions = CString::new(permissions).unwrap();

    unsafe {
        return match ffi::unveil(cpath.as_ptr(), cpermissions.as_ptr()) {
            0 => Ok(()),
            _ => Err(io::Error::last_os_error().raw_os_error().unwrap()),
        };
    }
}
