#![allow(non_snake_case)]

use objc2_core_foundation::{
    CFAllocator, CFArray, CFIndex, CFOptionFlags, CFRetained, CFString, CFType, Type,
};
use std::{ffi::c_void, ptr::NonNull};

#[repr(C)]
pub(super) struct CoreMDQuery([u8; 0]);

unsafe impl Type for CoreMDQuery {}

#[repr(C)]
pub(super) struct CoreMDItem([u8; 0]);

unsafe impl Type for CoreMDItem {}

// https://developer.apple.com/documentation/coreservices/1413029-mdquerycreate?language=objc
#[inline]
pub(super) unsafe extern "C-unwind" fn MDQueryCreate(
    allocator: Option<&CFAllocator>,
    query_string: &CFString,
    value_list_attrs: Option<&CFArray>,
    sorting_attrs: Option<&CFArray>,
) -> Option<CFRetained<CoreMDQuery>> {
    extern "C-unwind" {
        fn MDQueryCreate(
            allocator: Option<&CFAllocator>,
            query_string: Option<&CFString>,
            value_list_attrs: Option<&CFArray>,
            sorting_attrs: Option<&CFArray>,
        ) -> Option<NonNull<CoreMDQuery>>;
    }
    let ret = unsafe {
        MDQueryCreate(
            allocator,
            Some(query_string),
            value_list_attrs,
            sorting_attrs,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub(super) unsafe extern "C-unwind" fn MDItemCreate(
    allocator: Option<&CFAllocator>,
    path: &CFString,
) -> Option<CFRetained<CoreMDItem>> {
    extern "C-unwind" {
        fn MDItemCreate(
            allocator: Option<&CFAllocator>,
            path: Option<&CFString>,
        ) -> Option<NonNull<CoreMDItem>>;
    }
    let ret = unsafe { MDItemCreate(allocator, Some(path)) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub(super) unsafe extern "C-unwind" fn MDItemCopyAttributeNames(
    item: &CoreMDItem,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn MDItemCopyAttributeNames(item: &CoreMDItem) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { MDItemCopyAttributeNames(item) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub(super) unsafe extern "C-unwind" fn MDItemCopyAttribute(
    item: &CoreMDItem,
    name: &CFString,
) -> Option<CFRetained<CFType>> {
    extern "C-unwind" {
        fn MDItemCopyAttribute(item: &CoreMDItem, name: &CFString) -> Option<NonNull<CFType>>;
    }
    let ret = unsafe { MDItemCopyAttribute(item, name) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
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
    pub(super) fn MDQueryGetResultAtIndex(query: &CoreMDQuery, index: CFIndex) -> *const c_void;
}
