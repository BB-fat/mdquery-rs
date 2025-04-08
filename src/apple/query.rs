use super::{MDItem, MDQueryBuilder, MDQueryScope};
use anyhow::{anyhow, Result};
use objc2_core_foundation::{
    CFAllocator, CFArray, CFArrayCreate, CFIndex, CFOptionFlags, CFRetained, CFString,
};
use std::cell::UnsafeCell;
use std::marker::{PhantomData, PhantomPinned};
use std::ptr;
use std::sync::Arc;

#[repr(C)]
pub(super) struct CoreMDQuery {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    // https://developer.apple.com/documentation/coreservices/1413029-mdquerycreate?language=objc
    fn MDQueryCreate(
        allocator: Option<&CFAllocator>,
        query_string: &CFString,
        value_list_attrs: Option<&CFArray>,
        sorting_attrs: Option<&CFArray>,
    ) -> Option<CFRetained<CoreMDQuery>>;

    // https://developer.apple.com/documentation/coreservices/1413048-mdquerysetsearchscope?language=objc
    fn MDQuerySetSearchScope(
        query: &CoreMDQuery,
        scope_directories: &CFArray,
        scope_options: u32, // OptionBits
    );

    // https://developer.apple.com/documentation/coreservices/1413085-mdquerysetmaxcount?language=objc
    fn MDQuerySetMaxCount(query: &CoreMDQuery, max_count: CFIndex);

    // https://developer.apple.com/documentation/coreservices/1413099-mdqueryexecute?language=objc
    fn MDQueryExecute(query: &CoreMDQuery, option_flags: CFOptionFlags) -> bool;

    // https://developer.apple.com/documentation/coreservices/1413008-mdquerygetresultcount?language=objc
    fn MDQueryGetResultCount(query: &CoreMDQuery) -> CFIndex;
}

pub struct MDQuery(Arc<CFRetained<CoreMDQuery>>);

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

        Ok(MDQuery(Arc::new(md_query)))
    }

    pub fn execute(self) -> Result<Vec<MDItem>> {
        unsafe {
            // TODO 写成异步执行
            let success =
                MDQueryExecute(&self.0, MDQueryOptionsFlags::SYNCHRONOUS as CFOptionFlags);
            if !success {
                return Err(anyhow!("MDQuery execute failed."));
            }
            let count = MDQueryGetResultCount(&self.0);
            let mut items = Vec::with_capacity(count as usize);
            for i in 0..count {
                items.push(MDItem::new(i as isize, self.0.clone()));
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
    use super::*;
    use objc2_core_foundation::CFString;

    #[test]
    fn test_md_query_create() {
        let query_string = CFString::from_str("kMDItemFSName = \"test\"");
        let result = unsafe {
            MDQueryCreate(
                None, // kCFAllocatorDefault
                &query_string,
                None,
                None,
            )
        };
        assert!(result.is_some(), "MDQueryCreate should not return null");
    }

    #[test]
    fn test_md_query_new() {
        let result = MDQuery::new(
            "kMDItemFSName = \"test\"",
            Some(vec![MDQueryScope::Home]),
            None,
        );
        assert!(result.is_ok(), "MDQuery::new should not return error");
    }

    #[test]
    fn test_md_query_execute() {
        let query = MDQuery::new(
            "kMDItemFSName = \"Desktop\"",
            Some(vec![MDQueryScope::Home]),
            Some(5),
        )
        .unwrap();
        let items = query.execute().unwrap();
        assert!(items.len() > 0, "MDQuery::execute should return items");
    }
}
