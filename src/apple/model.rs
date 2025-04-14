use std::{fmt::{self, Display}, path::{Path, PathBuf}};

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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self::Custom(path.as_ref().to_path_buf())
    }

    pub(crate) fn into_scope_string(self) -> String {
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

/// Metadata attribute keys that can be used in queries.
///
/// These keys correspond to macOS Spotlight metadata attributes.
pub enum MDItemKey {
    /// The user-visible display name of the item
    DisplayName,
    /// The filename of the item
    FSName,
    /// The date the item's content was last modified
    ModificationDate,
    /// The date the item's content was created
    CreationDate,
    /// The date the item was last used/opened
    LastUsedDate,
    /// The size of the item in bytes
    Size,
    /// The UTI (Uniform Type Identifier) of the item
    ContentType,
    /// The path of the item
    Path,
}

impl MDItemKey {
    /// Returns the Spotlight API string representation of the key.
    ///
    /// # Returns
    /// The string constant used by the Spotlight API for this key.
    pub fn as_str(&self) -> &'static str {
        match self {
            MDItemKey::DisplayName => "kMDItemDisplayName",
            MDItemKey::FSName => "kMDItemFSName",
            MDItemKey::ModificationDate => "kMDItemContentModificationDate",
            MDItemKey::CreationDate => "kMDItemContentCreationDate",
            MDItemKey::LastUsedDate => "kMDItemLastUsedDate",
            MDItemKey::Size => "kMDItemFSSize",
            MDItemKey::ContentType => "kMDItemContentType",
            MDItemKey::Path => "kMDItemPath",
        }
    }

    /// Checks if this key represents a date/time attribute.
    ///
    /// # Returns
    /// `true` if this key is a time-related attribute, `false` otherwise.
    pub fn is_time(&self) -> bool {
        matches!(
            self,
            MDItemKey::ModificationDate | MDItemKey::CreationDate | MDItemKey::LastUsedDate
        )
    }
}

impl Display for MDItemKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
