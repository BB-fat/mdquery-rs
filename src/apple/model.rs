use std::path::PathBuf;
use anyhow::Result;

pub struct MDItem;

impl MDItem {
    pub fn get_attribute_names(&self) -> Vec<String> {
        unimplemented!()
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

pub enum MDQueryScope {
    Home,
    Computer,
    Network,
    AllIndexed,
    ComputerIndexed,
    NetworkIndexed,
    Custom(PathBuf)
}