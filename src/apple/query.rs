use super::api::*;
use super::{MDItem, MDQueryBuilder, MDQueryScope};
use anyhow::{anyhow, Result};
use objc2_core_foundation::{CFArrayCreate, CFIndex, CFRetained, CFString};
use std::ptr;

pub struct MDQuery(CFRetained<CoreMDQuery>);

impl MDQuery {
    pub fn builder() -> MDQueryBuilder {
        MDQueryBuilder::default()
    }

    pub fn new(
        query: &str,
        scopes: Option<Vec<MDQueryScope>>,
        max_count: Option<usize>,
    ) -> Result<Self> {
        let query = CFString::from_str(query);

        let md_query = unsafe {
            MDQueryCreate(
                None, // kCFAllocatorDefault
                &query, None, None,
            )
        }
        .ok_or(anyhow!("MDQuery create failed, check query syntax."))?;

        if let Some(scopes) = scopes {
            let scopes = scopes
                .into_iter()
                .map(|scope| scope.into_scope_string())
                .map(|scope| CFString::from_str(&scope))
                .collect::<Vec<_>>();

            let scopes = unsafe {
                CFArrayCreate(
                    None,
                    scopes.as_ptr() as *mut _,
                    scopes.len() as CFIndex,
                    ptr::null(),
                )
            }
            .ok_or(anyhow!("MDQuery create failed when create scope array."))?;

            unsafe {
                MDQuerySetSearchScope(&md_query, &scopes, 0);
            }
        }

        if let Some(max_count) = max_count {
            unsafe {
                MDQuerySetMaxCount(&md_query, max_count as CFIndex);
            }
        }

        Ok(MDQuery(md_query))
    }

    pub fn execute(self) -> Result<Vec<MDItem>> {
        unsafe {
            let query = self.0.clone();
            let success = MDQueryExecute(&query, MDQueryOptionsFlags::SYNCHRONOUS as _);

            if !success {
                return Err(anyhow!("MDQuery execute failed."));
            }

            let count = MDQueryGetResultCount(&self.0);
            let mut items = Vec::with_capacity(count as usize);
            for i in 0..count {
                if let Some(item) = MDQueryGetResultAtIndex(&self.0, i) {
                    items.push(MDItem::new(item));
                }
            }
            Ok(items)
        }
    }
}

// https://developer.apple.com/documentation/coreservices/mdqueryoptionflags?language=objc
#[repr(C)]
struct MDQueryOptionsFlags(u32);

#[allow(unused)]
impl MDQueryOptionsFlags {
    const NONE: u32 = 0;
    const SYNCHRONOUS: u32 = 1;
    const WANTS_UPDATES: u32 = 4;
    const ALLOW_FS_TRANSLATIONS: u32 = 8;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_md_query_execute() {
        let query = MDQuery::new(
            "kMDItemFSName = \"Safari.app\"",
            Some(vec![MDQueryScope::Custom("/Applications".into())]),
            Some(5),
        )
        .unwrap();

        let items = query.execute().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].path().unwrap(),
            PathBuf::from("/Applications/Safari.app")
        );
    }
}
