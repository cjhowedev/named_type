//! This crate provides the `NamedType` trait. The `named_type_derive` crate
//! also provides the ability to automatically derive this trait using
//! `#[derive(NamedType)]`.
//!
//! # Examples
//!
//! You can derive `NamedType` for any struct or enum to get some obvious
//! generated names. This is the expected usage of this crate for most types.
//!
//! ```
//! extern crate named_type;
//! #[macro_use]
//! extern crate named_type_derive;
//!
//! use named_type::NamedType;
//!
//! #[derive(NamedType)]
//! struct MyStruct {}
//!
//! #[derive(NamedType)]
//! enum MyEnum {}
//!
//! fn main() {
//!     assert_eq!(MyStruct::type_name(), concat!(module_path!(), "::MyStruct"));
//!     assert_eq!(MyStruct::short_type_name(), "MyStruct");
//!
//!     assert_eq!(MyEnum::type_name(), concat!(module_path!(), "::MyEnum"));
//!     assert_eq!(MyEnum::short_type_name(), "MyEnum");
//! }
//! ```
//!
//! Since it's possible that short type names conflict, there is the option to
//! add a prefix or suffix to a generated name to reduce ambiguity. Note that
//! this only affects the short type name.
//!
//! ```
//! # extern crate named_type;
//! # #[macro_use]
//! # extern crate named_type_derive;
//! # use named_type::NamedType;
//! #[derive(NamedType)]
//! #[named_type(short_suffix = "_suffix")]
//! struct Suffixed {}
//!
//! #[derive(NamedType)]
//! #[named_type(short_prefix = "Pre")]
//! enum Prefixed {}
//!
//! # fn main() {
//! assert_eq!(Suffixed::type_name(), concat!(module_path!(), "::Suffixed"));
//! assert_eq!(Suffixed::short_type_name(), "Suffixed_suffix");
//!
//! assert_eq!(Prefixed::type_name(), concat!(module_path!(), "::Prefixed"));
//! assert_eq!(Prefixed::short_type_name(), "PrePrefixed");
//! # }
//! ```

#[allow(unused_imports)]
#[macro_use]
extern crate named_type_derive;

#[doc(hidden)]
pub use named_type_derive::*;


/// A trait for getting the name of a type
pub trait NamedType {
    /// Returns the canonical name with the fully qualified module name for the
    /// given type
    fn type_name() -> &'static str
    where
        Self: Sized;

    /// Returns a user-friendly short name for the given type
    fn short_type_name() -> &'static str
    where
        Self: Sized;
}
