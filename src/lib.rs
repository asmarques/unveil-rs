extern crate libc;

#[cfg(target_os = "openbsd")]
mod openbsd;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotSupported,
    Os(i32),
}

#[cfg(target_os = "openbsd")]
pub fn unveil(path: &str, permissions: &str) -> Result<(), Error> {
    openbsd::unveil(path, permissions).map_err(|code| Error::Os(code))
}

#[cfg(not(target_os = "openbsd"))]
#[allow(unused_variables)]
pub fn unveil(path: &str, permissions: &str) -> Result<(), Error> {
    return Err(Error::NotSupported);
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_unveil() {
        let result = unveil(".", "r");
        assert!(result.is_ok())
    }

    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn test_unveil_not_supported() {
        let result = unveil(".", "r");
        assert_eq!(result.unwrap_err(), Error::NotSupported)
    }
}
