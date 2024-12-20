use settings::ValidationError;

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error 
{
  #[error(transparent)]
  DeserializeError(#[from] serde_json::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  UtilitesError(#[from] utilites::error::Error),
  #[error("Ошибка, сервер ответил кодом `{}` вместо кода `{}` -> `{}`", .1, .0, .2.as_ref().unwrap_or(&"".to_owned()))]
  ///1 - ожидаемый код ответа сервера
  ///2 - полученный код от сервера
  StatusCodeError(u16, u16, Option<String>),
}

// impl std::fmt::Display for Error
// {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
//   {
//     match self 
//     {
//       Error::Io(io) => f.write_str(&io.to_string()),
//       Error::Other(oth) => f.write_str(&oth.to_string()),
//       Error::SettingsValidation(e) => f.write_str(&vec_to_str(&e)),
//       Error::ServiceErrors(e) => f.write_str(&e.join("\\r\\n")),
//       Error::HyperError(e) => f.write_str(&e.to_string()),
//       Error::HttpError(e) => f.write_str(&e.to_string()),
//       Error::RequestError(e) => f.write_str(e),
//     }
//   }
// }

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