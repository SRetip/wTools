//!
//! Client of API.
//!

/// Define a private namespace for all its items.
mod private
{

}

use format_tools::
{
  Fields,
  TableWithFields,
};
use std::borrow::Cow;

mod assistant_object;
mod file_data;

crate::mod_interface!
{
  exposed use
  {
    assistant_object::AssistantObjectWrap,
    file_data::FileDataWrap,
  };
}
