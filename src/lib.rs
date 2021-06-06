extern crate libc;

#[cfg(target_os = "openbsd")]
mod openbsd;

use std::ffi::NulError;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotSupported,
    Path(NulError),
    Permissions(NulError),
    Os(i32),
}

#[cfg(target_os = "openbsd")]
pub fn unveil(path: impl AsRef<[u8]>, permissions: &str) -> Result<(), Error> {
    use std::ffi::CString;

    let path = path.as_ref();

    // iff path is empty, pass (NULL, NULL) to lock unveil(2). POSIX
    // doesn't allow empty pathnames, and unveil(2) assigns no other
    // meaning to empty path as of OpenBSD 6.8, so this is safe.
    if path.is_empty() {
        return openbsd::unveil(None, None).map_err(Error::Os);
    }

    // empty permissions means "deny all operations on path", which
    // is useful to override an ancestor's allowed operations. there
    // is no meaning for (non-NULL, NULL) as of OpenBSD 6.8.
    let path = CString::new(path).map_err(Error::Path)?;
    let permissions = CString::new(permissions).map_err(Error::Permissions)?;

    openbsd::unveil(Some(&path), Some(&permissions)).map_err(Error::Os)
}

#[cfg(not(target_os = "openbsd"))]
#[allow(unused_variables)]
pub fn unveil(path: impl AsRef<[u8]>, permissions: &str) -> Result<(), Error> {
    Err(Error::NotSupported)
}

#[cfg(test)]
mod tests {
    use *;

    // all tests for OpenBSD targets should live in one test function,
    // because the ones that affect process state need to run in a
    // specific order. with separate test functions, the tests would
    // run in a random order (by default) or in lexicographic order
    // of test names (cargo test -- --test-threads=1).
    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_unveil() {
        assert_eq!(unveil(".", "r"), Ok(()), "simple unveil should succeed");

        // null(4) is a file, and FFh is a valid filename (despite
        // being invalid UTF-8), so this should only throw ENOTDIR
        assert_eq!(
            unveil(b"/dev/null/\xFF", "r").unwrap_err(),
            Error::Os(libc::ENOTDIR),
            "unveil binary path under regular file should throw ENOTDIR",
        );

        assert_eq!(
            unveil("/dev/null", ""),
            Ok(()),
            "unveil child with empty permissions should succeed",
        );

        assert_eq!(unveil("/dev", "r"), Ok(()), "unveil parent should succeed");

        assert_eq!(
            unveil("", ""),
            Ok(()),
            "unveil empty strings should lock successfully",
        );

        assert_eq!(
            unveil(".", "r").unwrap_err(),
            Error::Os(libc::EPERM),
            "simple unveil after locking should throw EPERM",
        );

        use std::fs::File;

        assert!(
            File::open("/dev/zero").is_ok(),
            "opening /dev/zero should succeed",
        );

        assert!(
            File::open("/dev/null").is_err(),
            "opening /dev/null should fail",
        );
    }

    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn test_unveil_not_supported() {
        assert_eq!(unveil(".", "r").unwrap_err(), Error::NotSupported);
    }
}
