use super::api::*;
use anyhow::{anyhow, Result};
use objc2_core_foundation::{
    CFArrayGetCount, CFArrayGetValueAtIndex, CFIndex, CFRetained, CFString,
};
use std::{
    path::{Path, PathBuf},
    ptr::NonNull,
};

pub struct MDItem(CFRetained<CoreMDItem>);

impl MDItem {
    pub(super) fn new(item: CFRetained<CoreMDItem>) -> Self {
        Self(item)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let path_str = CFString::from_str(&path.to_str().ok_or(anyhow!("Invalid path"))?);
        let item =
            unsafe { MDItemCreate(None, &path_str) }.ok_or(anyhow!("Failed to create MDItem"))?;
        Ok(Self(item))
    }

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

    pub fn get_attribute(&self, name: &str) -> Result<String> {
        unimplemented!()
    }

    pub fn path(&self) -> Option<PathBuf> {
        unimplemented!()
    }

    pub fn display_name(&self) -> Option<String> {
        unimplemented!()
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
}
