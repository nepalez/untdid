//! Proc-macro for generating UN/EDIFACT code list types
//! from the code list file (UNCL) of a UN/EDIFACT directory.

mod parse;
mod prelude;

use crate::parse::{Element, parse};
use crate::prelude::{
    LitStr, PathBuf, ToSnakeCase, TokenStream, TokenStream2, format_ident, parse_macro_input, quote,
};

// Writes an optional text as a token stream of an optional static string.
fn optional(value: &Option<String>) -> TokenStream2 {
    match value {
        Some(text) => quote! { Some(#text) },
        None => quote! { None },
    }
}

// Checks that the manifest of the calling crate declares the feature of every element.
fn check_features(manifest: &str, elements: &[Element]) {
    let declared: Vec<_> = manifest
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(name, _)| name.trim())
        .collect();
    let missing: Vec<_> = elements
        .iter()
        .filter(|e| !declared.contains(&e.feature.as_str()))
        .map(|e| format!("{} ({})", e.feature, e.number))
        .collect();
    assert!(
        missing.is_empty(),
        "the manifest declares no features for the code lists: {}",
        missing.join(", ")
    );
}

// Generates the type of the code list of one element under the feature of the element.
fn code_list(element: &Element) -> TokenStream2 {
    let feature = &element.feature;
    let ident = format_ident!("{}", element.type_name);
    let count = element.codes.len();
    let variants: Vec<_> = element
        .codes
        .iter()
        .map(|c| format_ident!("{}", c.variant))
        .collect();
    let codes: Vec<_> = element.codes.iter().map(|c| &c.code).collect();
    let names: Vec<_> = element.codes.iter().map(|c| &c.name).collect();
    let descriptions: Vec<_> = element
        .codes
        .iter()
        .map(|c| optional(&c.description))
        .collect();
    let variant_docs: Vec<_> = element
        .codes
        .iter()
        .map(|c| format!("{} (`{}`).", c.name, c.code))
        .collect();
    let type_doc = format!(
        "UN/EDIFACT data element {} «{}»: {}",
        element.number, element.name, element.description
    );
    let tests_module = format_ident!("{}_generated_tests", element.type_name.to_snake_case());
    let tests: Vec<_> = element
        .codes
        .iter()
        .map(|c| format_ident!("code_{}", c.code.to_ascii_lowercase()))
        .collect();

    quote! {
        #[doc = #type_doc]
        #[cfg(feature = #feature)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[non_exhaustive]
        pub enum #ident {
            #(
                #[doc = #variant_docs]
                #variants,
            )*
        }

        #[cfg(feature = #feature)]
        impl #ident {
            /// Returns all static metadata for this code.
            #[inline]
            #[must_use]
            pub const fn info(self) -> CodeInfo {
                match self {
                    #(
                        Self::#variants => CodeInfo {
                            code: #codes,
                            name: #names,
                            description: #descriptions,
                        },
                    )*
                }
            }

            /// The code of the directory.
            #[inline]
            #[must_use]
            pub const fn code(self) -> &'static str {
                self.info().code
            }

            /// The English name.
            #[inline]
            #[must_use]
            pub const fn name(self) -> &'static str {
                self.info().name
            }

            /// The description, if any.
            #[inline]
            #[must_use]
            pub const fn description(self) -> Option<&'static str> {
                self.info().description
            }

            /// Look up a code by its string representation (case-sensitive).
            #[inline]
            #[must_use]
            pub fn from_code(code: &str) -> Option<Self> {
                match code {
                    #( #codes => Some(Self::#variants), )*
                    _ => None,
                }
            }

            /// Look up a code by its string representation (case-insensitive).
            #[inline]
            #[cfg(feature = "case-insensitive")]
            #[must_use]
            pub fn from_code_ignore_case(code: &str) -> Option<Self> {
                Self::ALL
                    .iter()
                    .copied()
                    .find(|value| value.code().eq_ignore_ascii_case(code))
            }

            /// Every active code in the order of the directory.
            pub const ALL: &'static [Self; #count] = &[
                #( Self::#variants, )*
            ];
        }

        #[cfg(feature = #feature)]
        impl core::fmt::Display for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.code())
            }
        }

        #[cfg(feature = #feature)]
        impl core::str::FromStr for #ident {
            type Err = UnknownCode;

            fn from_str(s: &str) -> Result<Self, UnknownCode> {
                #[cfg(feature = "case-insensitive")]
                let result = Self::from_code_ignore_case(s);

                #[cfg(not(feature = "case-insensitive"))]
                let result = Self::from_code(s);

                result.ok_or_else(|| UnknownCode(s.to_owned()))
            }
        }

        #[cfg(feature = #feature)]
        impl<'a> TryFrom<&'a str> for #ident {
            type Error = UnknownCode;

            fn try_from(s: &'a str) -> Result<Self, UnknownCode> {
                s.parse()
            }
        }

        #[cfg(all(feature = #feature, feature = "serde"))]
        impl crate::prelude::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: crate::prelude::Serializer,
            {
                serializer.serialize_str(self.code())
            }
        }

        #[cfg(all(feature = #feature, feature = "serde"))]
        impl<'de> crate::prelude::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: crate::prelude::Deserializer<'de>,
            {
                let code = <&str as crate::prelude::Deserialize>::deserialize(deserializer)?;
                code.parse().map_err(<D::Error as crate::prelude::DeError>::custom)
            }
        }

        #[cfg(all(test, feature = #feature, feature = "generated-tests"))]
        mod #tests_module {
            use super::*;
            #(
                #[test]
                fn #tests() {
                    let value: #ident = #codes.parse().unwrap();
                    assert_eq!(value, #ident::#variants);
                    assert_eq!(value.code(), #codes);
                    assert_eq!(value.to_string(), #codes);
                    assert_eq!(#ident::from_code(#codes), Some(value));
                }
            )*
        }
    }
}

/// Generates a type for the code list of every data element of the code list file (UNCL)
/// of a UN/EDIFACT directory.
///
/// The path is relative to the manifest directory of the calling crate.
/// Every type stays behind a feature named after its data element,
/// and the manifest of the calling crate must declare all of them.
/// The generated code refers to `serde` through `crate::prelude`,
/// so the calling crate re-exports `Serialize`, `Serializer`, `Deserialize`,
/// `Deserializer`, and `DeError` there when it enables its `serde` feature.
///
/// Usage: `untdid!("UNCL.24A");`
#[proc_macro]
pub fn untdid(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr).value();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_dir = PathBuf::from(manifest_dir);
    let path = manifest_dir.join(path);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let manifest = std::fs::read_to_string(manifest_dir.join("Cargo.toml"))
        .unwrap_or_else(|e| panic!("cannot read the manifest of the calling crate: {e}"));

    let elements = parse(&text);
    check_features(&manifest, &elements);
    let code_lists: Vec<_> = elements.iter().map(code_list).collect();

    quote! {
        /// All metadata of a code.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct CodeInfo {
            /// The code of the directory (e.g. `"380"`).
            pub code: &'static str,
            /// The English name of the code.
            pub name: &'static str,
            /// The description of the code, if defined.
            pub description: Option<&'static str>,
        }

        /// Error returned when parsing an unrecognised UN/EDIFACT code.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct UnknownCode(pub String);

        impl core::fmt::Display for UnknownCode {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "unknown UN/EDIFACT code: {:?}", self.0)
            }
        }

        impl std::error::Error for UnknownCode {}

        #( #code_lists )*
    }
    .into()
}
