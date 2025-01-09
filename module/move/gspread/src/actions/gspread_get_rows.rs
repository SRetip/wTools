//!
//! Action for the command "rows".
//!
//! Retrieves all rows from the specified Google Sheet, excluding the header.
//!


mod private
{
  use crate::*;
  use client::SheetsType;
  use actions::gspread::
  {
    get_rows, 
    Result
  };
  use ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    sheet_name : &str
  ) -> Result< Vec< Vec < JsonValue > > >
  {
    match get_rows( hub, spreadsheet_id, sheet_name ).await
    {
      Ok( rows ) => Ok( rows ),
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use action;
}
