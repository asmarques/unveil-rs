use std::ffi::CString;
use std::io;
use std::ptr;

mod ffi {
    use std::os::raw::{c_char, c_int};

    extern "C" {
        pub fn unveil(path: *const c_char, permissions: *const c_char) -> c_int;
    }
}

pub fn unveil(path: impl AsRef<[u8]>, permissions: &str) -> Result<(), i32> {
    let path = path.as_ref();
    let cpath = CString::new(path).unwrap();
    let cpermissions = CString::new(permissions).unwrap();

    let cpath_ptr = if !path.is_empty() {
        cpath.as_ptr()
    } else {
        ptr::null()
    };

    let cpermissions_ptr = if !permissions.is_empty() {
        cpermissions.as_ptr()
    } else {
        ptr::null()
    };

    unsafe {
        match ffi::unveil(cpath_ptr, cpermissions_ptr) {
            0 => Ok(()),
            _ => Err(io::Error::last_os_error().raw_os_error().unwrap()),
        }
    }
}
