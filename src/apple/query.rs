use super::{MDQueryBuilder, MDItem, MDQueryScope};
use anyhow::Result;

pub struct MDQuery;

impl MDQuery {
    pub fn builder() -> MDQueryBuilder {
        MDQueryBuilder::default()
    }

    pub fn new(query: &str, scopes: Option<Vec<MDQueryScope>>, max_count: Option<usize>) -> Self {
        unimplemented!()
    }

    pub async fn execute(self) -> Result<Vec<MDItem>> {
        unimplemented!()
    }
}
