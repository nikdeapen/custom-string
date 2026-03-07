#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

pub use validation_error::*;

mod validation_error;

mod custom_string;
