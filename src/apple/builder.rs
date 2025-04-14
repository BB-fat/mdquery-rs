use super::{MDQuery, MDQueryScope};

use anyhow::Result;
use std::fmt;
use std::fmt::Display;

/// Builder for constructing MDQuery instances with a fluent interface.
///
/// This builder allows for creating complex metadata queries on macOS using
/// the Spotlight search technology. It provides methods to build search expressions
/// that can be combined with logical AND operations.
///
/// # Examples
///
/// ```
/// use mdquery_rs::{MDQueryBuilder, MDQueryScope};
///
/// // Find files containing "document" in their name
/// let query = MDQueryBuilder::default()
///     .name_like("document")
///     .build(vec![MDQueryScope::Home], None)
///     .unwrap();
///     
/// let results = query.execute().unwrap();
/// for item in results {
///     println!("{:?}", item.path());
/// }
/// ```
#[derive(Default)]
pub struct MDQueryBuilder {
    expressions: Vec<String>,
}

impl MDQueryBuilder {
    /// Builds the final MDQuery with the current expressions.
    ///
    /// # Parameters
    /// * `scopes` - List of search scopes to apply (e.g., Home, Computer)
    /// * `max_count` - Optional maximum number of results to return
    ///
    /// # Returns
    /// A Result containing the MDQuery if successful, or an error if no expressions were added.
    ///
    /// # Errors
    /// Returns an error if no expressions were added to the builder.
    pub fn build(self, scopes: Vec<MDQueryScope>, max_count: Option<usize>) -> Result<MDQuery> {
        if self.expressions.is_empty() {
            anyhow::bail!("No expressions to build");
        }
        let query = self.gen_query();
        MDQuery::new(&query, Some(scopes), max_count)
    }

    /// Generates the final query string by joining all expressions with AND operators.
    ///
    /// # Returns
    /// A string representation of the combined query.
    fn gen_query(&self) -> String {
        self.expressions
            .iter()
            .map(|e| format!("({})", e))
            .collect::<Vec<_>>()
            .join(" && ")
    }

    /// Adds an expression to match items whose display name contains the specified string.
    ///
    /// This performs a case-insensitive substring search and supports Chinese Pinyin.
    ///
    /// # Parameters
    /// * `name` - The substring to match in display names
    ///
    /// # Returns
    /// Self for method chaining
    pub fn name_like(mut self, name: &str) -> Self {
        self.expressions
            .push(format!("{} == \"*{}*\"w", MDItemKey::DisplayName, name));
        self
    }

    /// Adds an expression to match items whose display name exactly matches the specified string.
    ///
    /// This performs a case-insensitive exact match.
    ///
    /// # Parameters
    /// * `name` - The exact name to match
    ///
    /// # Returns
    /// Self for method chaining
    pub fn name_is(mut self, name: &str) -> Self {
        self.expressions
            .push(format!("{} == \"{}\"c", MDItemKey::DisplayName, name));
        self
    }

    /// Adds a time-based comparison expression.
    ///
    /// # Parameters
    /// * `key` - The time-related metadata key to compare
    /// * `op` - The comparison operator to use
    /// * `timestamp` - Unix timestamp to compare against
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Panics
    /// Panics if the provided key is not a time-related key.
    pub fn time(mut self, key: MDItemKey, op: MDQueryCompareOp, timestamp: i64) -> Self {
        if !key.is_time() {
            panic!("Cannot use time on non-time key");
        }

        let time_str = chrono::DateTime::from_timestamp(timestamp, 0)
            .unwrap()
            .to_rfc3339();

        self.expressions.push(format!(
            "{} {} $time.iso({})",
            key,
            op.into_query_string(),
            time_str
        ));
        self
    }

    /// Adds a file size comparison expression.
    ///
    /// # Parameters
    /// * `op` - The comparison operator to use
    /// * `size` - The file size in bytes to compare against
    ///
    /// # Returns
    /// Self for method chaining
    pub fn size(mut self, op: MDQueryCompareOp, size: u64) -> Self {
        self.expressions.push(format!(
            "{} {} {}",
            MDItemKey::Size,
            op.into_query_string(),
            size
        ));
        self
    }

    /// Adds an expression to filter items based on whether they are directories.
    ///
    /// # Parameters
    /// * `value` - If true, matches only directories; if false, matches only non-directories
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Note
    /// Special directory types such as app bundles are not included in the directory scope.
    pub fn is_dir(mut self, value: bool) -> Self {
        self.expressions.push(format!(
            "{} {} \"{}\"",
            MDItemKey::ContentType,
            if value { "==" } else { "!=" },
            "public.folder"
        ));
        self
    }

    /// Adds an expression to filter items based on whether they are application bundles.
    ///
    /// # Returns
    /// Self for method chaining
    pub fn is_app(self) -> Self {
        self.content_type("com.apple.application-bundle")
    }

    /// Adds an expression to match items with the specified file extension.
    ///
    /// # Parameters
    /// * `ext` - The file extension to match (without the leading dot)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn extension(mut self, ext: &str) -> Self {
        self.expressions
            .push(format!("{} == \"*.{}\"c", MDItemKey::FSName, ext));
        self
    }

    /// Adds an expression to match items with the specified content type.
    ///
    /// # Parameters
    /// * `content_type` - The content type UTI string to match
    ///
    /// # Returns
    /// Self for method chaining
    pub fn content_type(mut self, content_type: &str) -> Self {
        self.expressions.push(format!(
            "{} == \"{}\"",
            MDItemKey::ContentType,
            content_type
        ));
        self
    }
}

/// Comparison operators for metadata query expressions.
pub enum MDQueryCompareOp {
    /// Greater than (>)
    GreaterThan,
    /// Less than (<)
    LessThan,
    /// Equal to (==)
    Equal,
    /// Greater than or equal to (>=)
    GreaterThanOrEqual,
    /// Less than or equal to (<=)
    LessThanOrEqual,
}

impl MDQueryCompareOp {
    /// Converts the operator to its string representation in a query.
    ///
    /// # Returns
    /// The string representation of the operator.
    fn into_query_string(self) -> &'static str {
        match self {
            MDQueryCompareOp::GreaterThan => ">",
            MDQueryCompareOp::LessThan => "<",
            MDQueryCompareOp::Equal => "==",
            MDQueryCompareOp::GreaterThanOrEqual => ">=",
            MDQueryCompareOp::LessThanOrEqual => "<=",
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_name_like() {
        let builder = MDQueryBuilder::default().name_like("Safari");
        let query = builder
            .build(vec![MDQueryScope::Computer], Some(1))
            .unwrap();
        let results = query.execute().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].path(),
            Some(PathBuf::from("/Applications/Safari.app"))
        );
    }

    #[test]
    fn test_is_app() {
        let builder = MDQueryBuilder::default().name_like("Safari").is_app();
        let query = builder
            .build(vec![MDQueryScope::Computer], Some(1))
            .unwrap();
        let results = query.execute().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].path(),
            Some(PathBuf::from("/Applications/Safari.app"))
        );
    }

    #[test]
    fn test_extension() {
        let builder = MDQueryBuilder::default().extension("txt");
        let query = builder
            .build(vec![MDQueryScope::Computer], Some(1))
            .unwrap();
        let results = query.execute().unwrap();
        assert!(!results.is_empty());
        assert!(results[0]
            .path()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with(".txt"));
    }

    #[test]
    fn test_time_search() {
        let now = chrono::Utc::now().timestamp();
        let builder = MDQueryBuilder::default().time(
            MDItemKey::ModificationDate,
            MDQueryCompareOp::LessThan,
            now,
        );
        let query = builder
            .build(vec![MDQueryScope::from_path("/Applications")], Some(1))
            .unwrap();
        let results = query.execute().unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_size_filter() {
        let builder = MDQueryBuilder::default().size(MDQueryCompareOp::GreaterThan, 1024 * 1024); // > 1MB
        let query = builder
            .build(vec![MDQueryScope::Computer], Some(1))
            .unwrap();
        let results = query.execute().unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_is_dir() {
        let builder = MDQueryBuilder::default().is_dir(true);
        let query = builder
            .build(vec![MDQueryScope::Computer], Some(1))
            .unwrap();
        let results = query.execute().unwrap();
        assert!(!results.is_empty());
    }
}
