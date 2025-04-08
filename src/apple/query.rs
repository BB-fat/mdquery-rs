use super::{MDItem, MDQueryBuilder, MDQueryScope};
use anyhow::{anyhow, Result};
use objc2_core_foundation::{CFAllocator, CFArray, CFArrayCreate, CFIndex, CFRetained, CFString};
use std::cell::UnsafeCell;
use std::marker::{PhantomData, PhantomPinned};
use std::ptr;

#[repr(C)]
struct CoreMDQuery {
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
}

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

    pub async fn execute(self) -> Result<Vec<MDItem>> {
        unimplemented!()
    }
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
}
