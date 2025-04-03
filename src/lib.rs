#[cfg(target_vendor = "apple")]
mod query;
#[cfg(target_vendor = "apple")]
mod builder;
#[cfg(target_vendor = "apple")]
mod model;

#[cfg(target_vendor = "apple")]
pub use query::*;
#[cfg(target_vendor = "apple")]
pub use builder::*;
#[cfg(target_vendor = "apple")]
pub use model::*;
