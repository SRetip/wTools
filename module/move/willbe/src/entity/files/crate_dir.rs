use crate::*;

use entity::
{
  PathError,
  ManifestFile,
};
use core::
{
  fmt,
  ops::
  {
    Deref,
    DerefMut,
  },
};
use std::
{
  path::{ Path, PathBuf },
  io,
};
use wtools::error::
{
  Result,
};
use path::{ AbsolutePath, Utf8Path };

// xxx : make generic

/// Path to crate directory
// #[ derive( Debug, Clone ) ]
#[ derive( Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
pub struct CrateDir( AbsolutePath );

impl CrateDir
{
  // qqq : bad : for Petro : why clone?
  // /// Returns an absolute path.
  // pub fn absolute_path( &self ) -> AbsolutePath
  // {
  //   self.0.clone()
  // }

  /// Returns inner type whicj is an absolute path.
  #[ inline( always ) ]
  pub fn inner( self ) -> AbsolutePath
  {
    self.0
  }

  /// Returns path to manifest aka cargo file.
  #[ inline( always ) ]
  pub fn manifest_file( self ) -> ManifestFile
  {
    self.into()
  }

}

impl fmt::Display for CrateDir
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "crate dir :: {}", self.0.display() )
  }
}

// impl AsRef< AbsolutePath > for CrateDir
// {
//   fn as_ref( &self ) -> &AbsolutePath
//   {
//     &self.0
//   }
// }

impl From< ManifestFile > for CrateDir
{
  fn from( src : ManifestFile ) -> Self
  {
    Self ( src.inner().parent().unwrap() )
  }
}

impl From< CrateDir > for AbsolutePath
{
  fn from( src : CrateDir ) -> Self
  {
    src.inner()
  }
}

impl From< CrateDir > for PathBuf
{
  fn from( src : CrateDir ) -> Self
  {
    src.inner().inner()
  }
}

impl< 'a > TryFrom< &'a CrateDir > for &'a str
{
  type Error = std::io::Error;
  fn try_from( src : &'a CrateDir ) -> Result< &'a str, Self::Error >
  {
    ( &src.0 ).try_into()
  }
}

impl TryFrom< &CrateDir > for String
{
  type Error = std::io::Error;
  fn try_from( src : &CrateDir ) -> Result< String, Self::Error >
  {
    let src2 : &str = src.try_into()?;
    Ok( src2.into() )
  }
}

impl TryFrom< AbsolutePath > for CrateDir
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : AbsolutePath ) -> Result< Self, Self::Error >
  {
    if !crate_dir_path.as_ref().join( "Cargo.toml" ).is_file()
    {
      let err =  io::Error::new( io::ErrorKind::InvalidData, format!( "Cannot find crate dir at {crate_dir_path:?}" ) );
      return Err( PathError::Io( err ) );
    }
    Ok( Self( crate_dir_path ) )
  }
}

impl TryFrom< PathBuf > for CrateDir
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl TryFrom< &Path > for CrateDir
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : &Path ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

// impl< AsUtf8 > TryFrom< AsUtf8 > for CrateDir
// where
//   AsUtf8 : AsRef< Utf8Path >,
// {
//   type Error = PathError;
//   #[ inline( always ) ]
//   fn try_from( crate_dir_path : AsUtf8 ) -> Result< Self, Self::Error >
//   {
//     Self::try_from( AbsolutePath::try_from( crate_dir_path.as_ref() )? )
//   }
// }

impl TryFrom< Utf8PathBuf > for CrateDir
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : Utf8PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl TryFrom< &Utf8Path > for CrateDir
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : &Utf8Path ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl AsRef< Path > for CrateDir
{
  fn as_ref( &self ) -> &Path
  {
    self.0.as_ref()
  }
}

impl AsMut< Path > for CrateDir
{
  fn as_mut( &mut self ) -> &mut Path
  {
    self.0.as_mut()
  }
}

impl Deref for CrateDir
{
  type Target = AbsolutePath;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl DerefMut for CrateDir
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}