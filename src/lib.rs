#[cfg(any(target_vendor = "apple", docsrs))]
mod apple;

#[cfg(any(target_vendor = "apple", docsrs))]
pub use apple::*;
