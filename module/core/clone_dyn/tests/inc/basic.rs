//
// #[ allow( unused_imports ) ]
// use super::*;
//
// use the_module::clone_dyn;
//
// #[ clone_dyn ]
// trait Trait1
// {
//   fn val( &self ) -> i32;
// }
//
// //
//
// impl Trait1 for i32
// {
//   fn val( &self ) -> i32
//   {
//     self.clone()
//   }
// }
//
// impl Trait1 for i64
// {
//   fn val( &self ) -> i32
//   {
//     self.clone().try_into().unwrap()
//   }
// }
//
// impl Trait1 for String
// {
//   fn val( &self ) -> i32
//   {
//     self.len().try_into().unwrap()
//   }
// }
//
// // impl< T > Trait1 for [ T ]
// // where
// //   T : clone_dyn::CloneDyn,
// // {
// //   fn val( &self ) -> i32
// //   {
// //     self.len()
// //   }
// // }
// //
// // impl Trait1 for str
// // {
// //   fn val( &self ) -> i32
// //   {
// //     self.len()
// //   }
// // }
//
// // include!( "./only_test/basic.rs" );
