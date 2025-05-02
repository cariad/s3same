#[doc = include_str!("../README.md")]

use std::{fs, io::Error, path::Path};

pub trait LocalPath {
    fn is_size(&self, size: u64) -> Result<bool, Error>;
}

impl LocalPath for Path {
    /// Checks if a local file is of an expected size.
    ///
    /// ```rust
    /// use s3same::LocalPath;
    ///
    /// let result = std::path::Path::new("LICENSE")
    ///     .is_size(1073)
    ///     .unwrap();
    ///
    /// assert!(result);
    /// ```
    fn is_size(&self, size: u64) -> Result<bool, std::io::Error> {
        match fs::metadata(self) {
            Ok(metadata) => Ok(metadata.len() == size),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LocalPath;

    #[test]
    fn is_size_false() {
        let result = std::path::Path::new("LICENSE").is_size(1072).unwrap();
        assert!(!result);
    }

    #[test]
    fn is_size_true() {
        let result = std::path::Path::new("LICENSE").is_size(1073).unwrap();
        assert!(result);
    }
}
