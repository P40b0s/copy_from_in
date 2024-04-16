use settings::ValidationError;

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error 
{
  #[error(transparent)]
  Io(#[from] std::io::Error),
  Other(#[from] anyhow::Error),
  SettingsValidation(Vec<ValidationError>),
  ServiceErrors(Vec<String>),
  HyperError(#[from] hyper::Error),
}

impl std::fmt::Display for Error
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
  {
    match self 
    {
      Error::Io(io) => f.write_str(&io.to_string()),
      Error::Other(oth) => f.write_str(&oth.to_string()),
      Error::SettingsValidation(e) => f.write_str(&vec_to_str(&e)),
      Error::ServiceErrors(e) => f.write_str(&e.join("\\r\\n")),
      Error::HyperError(e) => f.write_str(&e.to_string()),
    }
  }
}

fn vec_to_str(val : &Vec<ValidationError>) -> String
{
  let mut errors = String::new();
  let error = val.iter().fold(&mut errors, |acc, val|
  {
      let str = [val.to_string(), "\\n".to_owned()].concat();
      acc.push_str(&str);
      acc
  });
  error.to_string()
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