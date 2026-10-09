//! Re-exports of external dependencies shared across the crate.

pub use heck::{ToKebabCase, ToPascalCase, ToSnakeCase};
pub use proc_macro::TokenStream;
pub use proc_macro2::TokenStream as TokenStream2;
pub use quote::{format_ident, quote};
pub use std::collections::{HashMap, HashSet};
pub use std::path::PathBuf;
pub use syn::{LitStr, parse_macro_input};
