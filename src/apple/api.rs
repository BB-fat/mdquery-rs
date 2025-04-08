use objc2_core_foundation::{
    CFAllocator, CFArray, CFIndex, CFOptionFlags, CFRetained, CFString,
};
use std::cell::UnsafeCell;
use std::marker::{PhantomData, PhantomPinned};

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    // https://developer.apple.com/documentation/coreservices/1413029-mdquerycreate?language=objc
    pub(super) fn MDQueryCreate(
        allocator: Option<&CFAllocator>,
        query_string: &CFString,
        value_list_attrs: Option<&CFArray>,
        sorting_attrs: Option<&CFArray>,
    ) -> Option<CFRetained<CoreMDQuery>>;

    // https://developer.apple.com/documentation/coreservices/1413048-mdquerysetsearchscope?language=objc
    pub(super) fn MDQuerySetSearchScope(
        query: &CoreMDQuery,
        scope_directories: &CFArray,
        scope_options: u32, // OptionBits
    );

    // https://developer.apple.com/documentation/coreservices/1413085-mdquerysetmaxcount?language=objc
    pub(super) fn MDQuerySetMaxCount(query: &CoreMDQuery, max_count: CFIndex);

    // https://developer.apple.com/documentation/coreservices/1413099-mdqueryexecute?language=objc
    pub(super) fn MDQueryExecute(query: &CoreMDQuery, option_flags: CFOptionFlags) -> bool;

    // https://developer.apple.com/documentation/coreservices/1413008-mdquerygetresultcount?language=objc
    pub(super) fn MDQueryGetResultCount(query: &CoreMDQuery) -> CFIndex;

    // https://developer.apple.com/documentation/coreservices/1413055-mdquerygetresultatindex?language=objc
    pub(super) fn MDQueryGetResultAtIndex(query: &CoreMDQuery, index: CFIndex) -> Option<CFRetained<CoreMDItem>>;
}

#[repr(C)]
pub(super) struct CoreMDQuery {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[repr(C)]
pub(super) struct CoreMDItem {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}
