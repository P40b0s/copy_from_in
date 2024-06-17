use futures::FutureExt;
use settings::ValidationError;

#[derive(Debug, thiserror::Error)]
pub enum Error 
{
  #[error(transparent)]
  Io(#[from] std::io::Error),
  Other(#[from] anyhow::Error),
  SettingsValidation(Vec<ValidationError>),
  ServiceErrors(Vec<String>),
  HyperError(#[from] hyper::Error),
  //Ошибка если дата и размер копируемого файла не может синхронизироваться больше 2 минут
  FileTimeCopyError(String)
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
      Error::FileTimeCopyError(e) => f.write_str(&e),
    }
  }
}

impl From<Error> for futures::future::BoxFuture<'static, anyhow::Result<u64, Error>>
{
  fn from(value: Error) -> Self 
  {
    match value
    {
      Error::Io(io) => async move { Err(Error::Io(io)) }.boxed(),
      Error::Other(oth) => async move { Err(Error::Other(oth)) }.boxed(),
      Error::SettingsValidation(e) => async move { Err(Error::SettingsValidation(e)) }.boxed(),
      Error::ServiceErrors(e) => async move { Err(Error::ServiceErrors(e)) }.boxed(),
      Error::HyperError(e) => async move { Err(Error::HyperError(e)) }.boxed(),
      Error::FileTimeCopyError(e) => async move { Err(Error::FileTimeCopyError(e)) }.boxed(),
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