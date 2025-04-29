use super::{api::*, MDItemKey};
use anyhow::{anyhow, Result};
use objc2_core_foundation::{
    CFArray, CFArrayGetCount, CFArrayGetValueAtIndex, CFIndex, CFRetained, CFString, ConcreteType,
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
            .map(|name| (*name).to_string())
    }

    pub fn content_type(&self) -> Option<String> {
        self.get_attribute::<CFString>(MDItemKey::ContentType.as_str())
            .map(|name| (*name).to_string())
    }

    /// Retrieves the content type tree of this MDItem.
    ///
    /// # Returns
    /// * `Option<Vec<String>>` - The content type tree, or None if not available
    pub fn content_type_tree(&self) -> Option<Vec<String>> {
        self.get_attribute::<CFArray>(MDItemKey::ContentTypeTree.as_str())
            .map(|content_type_tree| {
                let count = unsafe { CFArrayGetCount(&content_type_tree) } as usize;
                let mut types = Vec::with_capacity(count);
                for i in 0..count {
                    let type_ptr =
                        unsafe { CFArrayGetValueAtIndex(&content_type_tree, i as CFIndex) };
                    if let Some(cf_type) = NonNull::new(type_ptr as *mut CFString) {
                        let type_str = unsafe { cf_type.as_ref().to_string() };
                        types.push(type_str);
                    }
                }
                types
            })
    }

    /// Checks if this MDItem is a directory.
    ///
    /// # Returns
    /// * `bool` - Returns true if this is a directory, false otherwise
    pub fn is_dir(&self) -> bool {
        self.content_type_tree()
            .map(|types| types.contains(&"public.folder".to_string()))
            .unwrap_or(false)
    }

    /// Checks if this MDItem is an image file.
    ///
    /// # Returns
    /// * `bool` - Returns true if this is an image file, false otherwise
    pub fn is_image(&self) -> bool {
        self.content_type_tree()
            .map(|types| types.contains(&"public.image".to_string()))
            .unwrap_or(false)
    }

    /// Checks if this MDItem is an application bundle.
    ///
    /// # Returns
    /// * `bool` - Returns true if this is an application bundle, false otherwise
    pub fn is_app(&self) -> bool {
        self.content_type()
            .map(|content_type| content_type == "com.apple.application-bundle")
            .unwrap_or(false)
    }

    /// Checks if this MDItem is a video file.
    ///
    /// # Returns
    /// * `bool` - Returns true if this is a video file, false otherwise
    pub fn is_video(&self) -> bool {
        self.content_type_tree()
            .map(|types| types.contains(&"public.movie".to_string()))
            .unwrap_or(false)
    }

    /// Checks if this MDItem is an audio file.
    ///
    /// # Returns
    /// * `bool` - Returns true if this is an audio file, false otherwise
    pub fn is_audio(&self) -> bool {
        self.content_type_tree()
            .map(|types| types.contains(&"public.audio".to_string()))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_attribute_names() {
        let item = MDItem::from_path("/Applications/Safari.app").unwrap();
        let names = item.get_attribute_names();
        assert!(!names.is_empty());
    }

    #[test]
    fn test_get_path() {
        let item = MDItem::from_path("/Applications/Safari.app").unwrap();
        let path = item.path().unwrap();
        assert_eq!(path, PathBuf::from("/Applications/Safari.app"));
    }

    #[test]
    fn test_get_content_type_tree() {
        let item = MDItem::from_path("/Applications/Safari.app").unwrap();
        let content_type_tree = item.content_type_tree().unwrap();
        assert!(!content_type_tree.is_empty());
    }
}
