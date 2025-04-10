use super::{MDQuery, MDQueryScope};

use anyhow::Result;
use std::fmt;
use std::fmt::Display;

#[derive(Default)]
pub struct MDQueryBuilder {
    expressions: Vec<String>,
}

impl MDQueryBuilder {
    pub fn build(self, scopes: Vec<MDQueryScope>, max_count: Option<usize>) -> Result<MDQuery> {
        if self.expressions.is_empty() {
            anyhow::bail!("No expressions to build");
        }
        let query = self.gen_query();
        MDQuery::new(&query, Some(scopes), max_count)
    }

    fn gen_query(&self) -> String {
        self.expressions
            .iter()
            .map(|e| format!("({})", e))
            .collect::<Vec<_>>()
            .join(" && ")
    }

    pub fn name_like(mut self, name: &str) -> Self {
        self.expressions
            .push(format!("{} == \"*{}*\"c", MDItemKey::DisplayName, name));
        self
    }

    pub fn name_is(mut self, name: &str) -> Self {
        self.expressions
            .push(format!("{} == \"{}\"c", MDItemKey::DisplayName, name));
        self
    }

    pub fn time(mut self, key: MDItemKey, op: MDQueryCompareOp, timestamp: i64) -> Self {
        if !key.is_time() {
            panic!("Cannot use time on non-time key");
        }

        let time_str = chrono::DateTime::from_timestamp(timestamp, 0)
            .unwrap()
            .to_rfc3339();

        self.expressions
            .push(format!("{} {} {}", key, op.into_query_string(), time_str));
        self
    }

    pub fn size(mut self, op: MDQueryCompareOp, size: u64) -> Self {
        self.expressions.push(format!(
            "{} {} {}",
            MDItemKey::Size,
            op.into_query_string(),
            size
        ));
        self
    }

    pub fn is_dir(mut self, value: bool) -> Self {
        self.expressions.push(format!(
            "{} {} \"{}\"c",
            MDItemKey::ContentType,
            if value { "==" } else { "!=" },
            "public.folder"
        ));
        self
    }

    pub fn extension(mut self, ext: &str) -> Self {
        self.expressions
            .push(format!("{} == \"*.{}\"c", MDItemKey::FSName, ext));
        self
    }

    pub fn content_type(mut self, content_type: &str) -> Self {
        self.expressions.push(format!(
            "{} == \"{}\"",
            MDItemKey::ContentType,
            content_type
        ));
        self
    }
}

pub enum MDQueryCompareOp {
    GreaterThan,
    LessThan,
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl MDQueryCompareOp {
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

pub enum MDItemKey {
    DisplayName,
    FSName,
    ModificationDate,
    CreationDate,
    LastUsedDate,
    Size,
    ContentType,
}

impl MDItemKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            MDItemKey::DisplayName => "kMDItemDisplayName",
            MDItemKey::FSName => "kMDItemFSName",
            MDItemKey::ModificationDate => "kMDItemContentModificationDate",
            MDItemKey::CreationDate => "kMDItemContentCreationDate",
            MDItemKey::LastUsedDate => "kMDItemLastUsedDate",
            MDItemKey::Size => "kMDItemFSSize",
            MDItemKey::ContentType => "kMDItemContentType",
        }
    }

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
    fn test_gen_query_name_like_and_name_is() {
        let builder = MDQueryBuilder::default().name_like("test").name_is("test");
        assert_eq!(
            builder.gen_query(),
            "(kMDItemDisplayName == \"*test*\"c) && (kMDItemDisplayName == \"test\"c)"
        );
    }

    #[test]
    fn test_find_safari_app() {
        let builder = MDQueryBuilder::default().name_like("浏览器");
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
}
