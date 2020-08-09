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
    openbsd::unveil(path, permissions).map_err(Error::Os)
}

#[cfg(not(target_os = "openbsd"))]
#[allow(unused_variables)]
pub fn unveil(path: &str, permissions: &str) -> Result<(), Error> {
    Err(Error::NotSupported)
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_unveil() {
        assert!(unveil(".", "r").is_ok());
    }

    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_unveil_restrict() {
        assert!(unveil("", "").is_ok());
        assert!(unveil(".", "r").is_err());
    }

    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn test_unveil_not_supported() {
        assert_eq!(unveil(".", "r").unwrap_err(), Error::NotSupported);
    }
}
