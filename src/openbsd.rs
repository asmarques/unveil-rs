use std::ffi::CStr;
use std::io;
use std::ptr;

mod ffi {
    use std::os::raw::{c_char, c_int};

    extern "C" {
        pub fn unveil(path: *const c_char, permissions: *const c_char) -> c_int;
    }
}

pub fn unveil(path: Option<&CStr>, permissions: Option<&CStr>) -> Result<(), i32> {
    let path = if let Some(path) = path {
        path.as_ptr()
    } else {
        ptr::null()
    };

    let permissions = if let Some(permissions) = permissions {
        permissions.as_ptr()
    } else {
        ptr::null()
    };

    unsafe {
        match ffi::unveil(path, permissions) {
            0 => Ok(()),
            _ => Err(io::Error::last_os_error().raw_os_error().unwrap()),
        }
    }
}
