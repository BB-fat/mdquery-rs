use std::path::PathBuf;
use anyhow::Result;
use objc2_core_foundation::CFRetained;
use std::sync::Arc;

use super::CoreMDQuery;

pub struct MDItem {
    index: isize,
    query: Arc<CFRetained<CoreMDQuery>>,
}

impl MDItem {
    pub(super) fn new(index: isize, query: Arc<CFRetained<CoreMDQuery>>) -> Self {
        Self { index, query }
    }

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

impl MDQueryScope {
    pub(crate) fn into_scope_string(&self) -> String {
        match self {
            MDQueryScope::Home => "kMDQueryScopeHome".to_string(),
            MDQueryScope::Computer => "kMDQueryScopeComputer".to_string(),
            MDQueryScope::Network => "kMDQueryScopeNetwork".to_string(),
            MDQueryScope::AllIndexed => "kMDQueryScopeAllIndexed".to_string(),
            MDQueryScope::ComputerIndexed => "kMDQueryScopeComputerIndexed".to_string(),
            MDQueryScope::NetworkIndexed => "kMDQueryScopeNetworkIndexed".to_string(),
            MDQueryScope::Custom(path) => path.to_string_lossy().to_string(),
        }
    }
}
