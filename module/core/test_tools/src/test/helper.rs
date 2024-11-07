
//!
//! Helpers for testing.
//!

// use super::*;

/// Define a private namespace for all its items.
mod private
{

  // zzz : move here test tools

  // /// Pass only if callback fails either returning error or panicing.
  //
  // pub fn should_throw< R, F : FnOnce() -> anyhow::Result< R > >( f : F ) -> anyhow::Result< R >
  // {
  //   f()
  // }

  //

  // #[panic_handler]
  // fn panic( info : &core::panic::PanicInfo ) -> !
  // {
  //   println!( "{:?}", info );
  //   loop {}
  // }

  // pub use test_suite;
  // pub use test_suite_internals;
  // pub use index;

  ///
  /// Required to convert integets to floats.
  ///

  #[ macro_export ]
  macro_rules! num
  {

    () =>
    {
    };

    ( $num : expr ) =>
    {
      num_traits::cast::< _, T >( $num ).unwrap()
    };

    ( $( $num : expr ),+ ) =>
    {(
      $( num_traits::cast::< _, T >( $num ).unwrap() ),+
    )};

  }

  ///
  /// Test a file with documentation.
  ///

  #[ macro_export ]
  macro_rules! doc_file_test
  {
    ( $file:expr ) =>
    {
      #[ allow( unused_doc_comments ) ]
      #[ cfg( doctest ) ]
      #[ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", $file ) ) ]
      extern { }
    };
  }

  pub use num;
  pub use doc_file_test;
}

// crate::mod_interface!
// {
//   // xxx
//   // #![ debug ]
//   // exposed use super;
//   exposed use super::super::helper;
//
//   prelude use
//   {
//     num,
//     doc_file_test,
//   };
// }

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use
  {
    private::*,
  };

}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  pub use super::super::helper;

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use
  {
    private::num,
    private::doc_file_test,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use
  {
  };

}
