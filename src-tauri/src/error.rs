// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  Other(#[from] anyhow::Error)
}

impl std::fmt::Display for Error
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
  {
      match self 
      {
        Error::Io(io) => f.write_str(&io.to_string()),
        Error::Other(oth) => f.write_str(&oth.to_string())
      }
  }
}



// we must manually implement serde::Serialize
impl serde::Serialize for Error 
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
  S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

#[tauri::command]
fn my_custom_command() -> Result<(), Error> {
  // This will return an error
  std::fs::File::open("path/that/does/not/exist")?;
  // Return nothing on success
  Ok(())
}