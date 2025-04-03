#[cfg(target_vendor = "apple")]
mod apple;

#[cfg(target_vendor = "apple")]
pub use apple::*;
