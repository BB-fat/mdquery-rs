use std::path::PathBuf;

pub enum MDQueryScope {
    Home,
    Computer,
    Network,
    AllIndexed,
    ComputerIndexed,
    NetworkIndexed,
    Custom(PathBuf),
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
