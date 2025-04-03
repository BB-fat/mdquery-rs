use super::{MDQueryBuilder, MDQueryItem};
use anyhow::Result;
use futures::stream::BoxStream;
use std::path::PathBuf;

pub struct MDQuery;

impl MDQuery {
    pub fn builder() -> MDQueryBuilder {
        MDQueryBuilder::default()
    }

    pub fn new(query: &str, scopes: Option<Vec<PathBuf>>, max_count: Option<usize>) -> Self {
        unimplemented!()
    }

    pub async fn execute(self) -> Result<Vec<MDQueryItem>> {
        unimplemented!()
    }

    pub fn watch(&self) -> Result<BoxStream<'_, Vec<MDQueryItem>>> {
        unimplemented!()
    }
}
