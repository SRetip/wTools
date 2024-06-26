//!
//! Attribute's properties. Reuse them to define how to parse properties of an attribute.
//!
//! # Example
//!
//! ```rust
//! use macro_tools::AttributePropertyBoolean;
//!
//! #[ derive( Debug, Default, Clone, Copy ) ]
//! pub struct DebugMarker;
//!
//! #[ derive( Debug, Default, Clone, Copy ) ]
//! pub struct EnabledMarker;
//!
//! pub trait AttributePropertyComponent
//! {
//!   const KEYWORD : &'static str;
//! }
//!
//! impl AttributePropertyComponent for DebugMarker
//! {
//!   const KEYWORD : &'static str = "debug";
//! }
//!
//! impl AttributePropertyComponent for EnabledMarker
//! {
//!   const KEYWORD : &'static str = "enabled";
//! }
//!
//! #[ derive( Debug, Default ) ]
//! struct MyAttributes
//! {
//!   pub debug : AttributePropertyBoolean< DebugMarker >,
//!   pub enabled : AttributePropertyBoolean< EnabledMarker >,
//! }
//!
//! impl syn::parse::Parse for MyAttributes
//! {
//!   fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
//!   {
//!     let mut debug = AttributePropertyBoolean::< DebugMarker >::default();
//!     let mut enabled = AttributePropertyBoolean::< EnabledMarker >::default();
//!
//!     while !input.is_empty()
//!     {
//!       let lookahead = input.lookahead1();
//!       if lookahead.peek( syn::Ident )
//!       {
//!         let ident : syn::Ident = input.parse()?;
//!         match ident.to_string().as_str()
//!         {
//!           DebugMarker::KEYWORD => debug = input.parse()?,
//!           EnabledMarker::KEYWORD => enabled = input.parse()?,
//!           _ => return Err( lookahead.error() ),
//!         }
//!       }
//!       else
//!       {
//!         return Err( lookahead.error() );
//!       }
//!
//!       // Optional comma handling
//!       if input.peek( syn::Token![,] )
//!       {
//!         input.parse::< syn::Token![,] >()?;
//!       }
//!     }
//!
//!     Ok( MyAttributes { debug, enabled } )
//!   }
//! }
//!
//! let input : syn::Attribute = syn::parse_quote!( #[ attribute( enabled = true ) ] );
//! let meta = match input.meta
//! {
//!   syn::Meta::List( meta_list ) => meta_list,
//!   _ => panic!( "Expected a Meta::List" ),
//! };
//!
//! let nested_meta_stream : proc_macro2::TokenStream = meta.tokens;
//! let attrs : MyAttributes = syn::parse2( nested_meta_stream ).unwrap();
//! println!( "{:?}", attrs );
//! ```
//!
//! In this example, the `AttributePropertyBoolean` struct is used to define attributes with boolean properties.
//! The `DebugMarker` and `EnabledMarker` structs act as markers to distinguish between different boolean attributes.
//! The `MyAttributes` struct aggregates these boolean attributes.
//!
//! The `Parse` implementation for `MyAttributes` iterates through the attribute's key-value pairs,
//! identifying each by its marker's keyword and parsing the boolean value.
//! It uses the `ParseStream` to parse identifiers and their associated values,
//! matching them to the appropriate marker's keyword.
//! If an unrecognized identifier is encountered, it returns an error.
//!
//! The `parse_quote!` macro is used to create a `syn::Attribute` instance with the attribute syntax,
//! which is then parsed into the `MyAttributes` struct. The resulting `MyAttributes` instance is printed to the console.

mod singletone;
mod singletone_optional;
mod boolean;
mod boolean_optional;
mod syn;
mod syn_optional;

/// Internal namespace.
pub( crate ) mod private
{
  // use crate::*;

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as attr_prop;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {

    singletone::AttributePropertySingletone,
    singletone::AttributePropertySingletoneMarker,
    singletone_optional::AttributePropertyOptionalSingletone,
    singletone_optional::AttributePropertyOptionalSingletoneMarker,
    boolean::AttributePropertyBoolean,
    boolean::AttributePropertyBooleanMarker,
    boolean_optional::AttributePropertyOptionalBoolean,
    boolean_optional::AttributePropertyOptionalBooleanMarker,
    syn::AttributePropertySyn,
    syn::AttributePropertySynMarker,
    syn_optional::AttributePropertyOptionalSyn,
    syn_optional::AttributePropertyOptionalSynMarker,

  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
