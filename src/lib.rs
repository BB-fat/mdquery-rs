#[cfg(any(target_vendor = "apple", doc))]
mod apple;

#[cfg(any(target_vendor = "apple", doc))]
pub use apple::*;
