mod dir;
pub use dir::*;

mod file;

#[doc(hidden)]
pub use embed_macros::__include_dir;
