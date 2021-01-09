extern crate libc;

#[cfg(target_os = "openbsd")]
mod openbsd;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotSupported,
    Os(i32),
}

#[cfg(target_os = "openbsd")]
pub fn unveil(path: impl AsRef<[u8]>, permissions: &str) -> Result<(), Error> {
    openbsd::unveil(path, permissions).map_err(Error::Os)
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
        assert_eq!(
            unveil(".", "r"),
            Ok(()),
            "simple unveil should succeed",
        );

        // null(4) is a file, and FFh is a valid filename (despite
        // being invalid UTF-8), so this should only throw ENOTDIR
        assert_eq!(
            unveil(b"/dev/null/\xFF", "r").unwrap_err(),
            Error::Os(libc::ENOTDIR),
            "unveil binary path under regular file should throw ENOTDIR",
        );

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
    }

    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn test_unveil_not_supported() {
        assert_eq!(unveil(".", "r").unwrap_err(), Error::NotSupported);
    }
}
