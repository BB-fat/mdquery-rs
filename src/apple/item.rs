use super::{api::*, MDItemKey};
use anyhow::{anyhow, Result};
use objc2_core_foundation::{
    CFArrayGetCount, CFArrayGetValueAtIndex, CFIndex, CFRetained, CFString, ConcreteType,
};
use std::{
    path::{Path, PathBuf},
    ptr::NonNull,
};

/// A wrapper around macOS Metadata Item (MDItem).
/// Provides access to file and directory metadata through the Spotlight metadata framework.
pub struct MDItem(CFRetained<CoreMDItem>);

impl MDItem {
    /// Creates a new MDItem from a file path.
    ///
    /// # Arguments
    /// * `path` - A path to a file or directory
    ///
    /// # Returns
    /// * `Result<Self>` - A new MDItem instance or an error
    ///
    /// # Errors
    /// * Returns an error if the path is invalid or if the MDItem creation fails
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let path_str = CFString::from_str(path.to_str().ok_or(anyhow!("Invalid path"))?);
        let item =
            unsafe { MDItemCreate(None, &path_str) }.ok_or(anyhow!("Failed to create MDItem"))?;
        Ok(Self(item))
    }

    /// Retrieves all available attribute names for this MDItem.
    ///
    /// # Returns
    /// * `Vec<String>` - A vector of attribute name strings
    pub fn get_attribute_names(&self) -> Vec<String> {
        unsafe { MDItemCopyAttributeNames(&self.0) }
            .map(|array| unsafe {
                let count = CFArrayGetCount(&array) as usize;
                let mut names = Vec::with_capacity(count);
                for i in 0..count {
                    let name_ptr = CFArrayGetValueAtIndex(&array, i as CFIndex);
                    if let Some(cf_name) = NonNull::new(name_ptr as *mut CFString) {
                        let name = cf_name.as_ref().to_string();
                        names.push(name);
                    }
                }
                names
            })
            .unwrap_or_default()
    }

    /// Gets a specific attribute from the MDItem.
    ///
    /// # Arguments
    /// * `name` - The name of the attribute to retrieve
    ///
    /// # Returns
    /// * `Option<CFRetained<T>>` - The attribute value cast to the specified type, or None if not available
    pub fn get_attribute<T: Sized + ConcreteType>(&self, name: &str) -> Option<CFRetained<T>> {
        let name = CFString::from_str(name);
        let value = unsafe { MDItemCopyAttribute(&self.0, &name) }?;
        value.downcast::<T>().ok()
    }

    /// Retrieves the file path of this MDItem.
    ///
    /// # Returns
    /// * `Option<PathBuf>` - The file path, or None if not available
    pub fn path(&self) -> Option<PathBuf> {
        self.get_attribute::<CFString>(MDItemKey::Path.as_str())
            .map(|path| {
                let path_str = (*path).to_string();
                PathBuf::from(path_str)
            })
    }

    /// Retrieves the display name of this MDItem.
    ///
    /// # Returns
    /// * `Option<String>` - The display name, or None if not available
    pub fn display_name(&self) -> Option<String> {
        self.get_attribute::<CFString>(MDItemKey::DisplayName.as_str())
            .map(|name| {
                
                (*name).to_string()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_attribute_names() {
        let item = MDItem::from_path("/Applications/Safari.app").unwrap();
        let names = item.get_attribute_names();
        assert!(names.len() > 0);
    }

    #[test]
    fn test_get_path() {
        let item = MDItem::from_path("/Applications/Safari.app").unwrap();
        let path = item.path().unwrap();
        assert_eq!(path, PathBuf::from("/Applications/Safari.app"));
    }
}
