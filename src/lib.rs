#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

pub use validation_error::*;

mod validation_error;

mod custom_string;

#[doc(hidden)]
pub use paste::paste as __paste;

#[cfg(feature = "serde")]
#[doc(hidden)]
pub use serde as __serde;
