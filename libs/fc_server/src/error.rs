use futures::FutureExt;
use settings::ValidationError;

#[derive(Debug, thiserror::Error)]
pub enum Error 
{
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    DbError(#[from] db_service::DbError),
    #[error(transparent)]
    RedbError(#[from] redb::Error),
    #[error(transparent)]
    RedbTransactionError(#[from] redb::TransactionError),
    #[error(transparent)]
    RedbTableError(#[from] redb::TableError),
    #[error(transparent)]
    RedbStorageError(#[from] redb::StorageError),
    #[error(transparent)]
    RedbCommitError(#[from] redb::CommitError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("Ошибка валидации настроек: {}", from_validation_error(.0))]
    SettingsValidation(Vec<ValidationError>),
    #[error("Ошибка валидации настроек: {}", from_service_error(.0))]
    ServiceErrors(Vec<String>),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    //Ошибка если дата и размер копируемого файла не может синхронизироваться больше 2 минут
    #[error("Превышено максимальное количество попыток при попытке копирования файла `{0}`, файл должен успевать копироваться в систему в течении 2 минут")]
    FileTimeCopyError(String),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}
impl serde::Serialize for Error 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

fn from_validation_error(err: &Vec<ValidationError>) -> String
{
    let mut out = String::new();
    for e in err
    {
        let str_err = e.to_string();
        out.push_str(&str_err);
    }
    out
}
fn from_service_error(err: &Vec<String>) -> String
{
    let mut out = String::new();
    for e in err
    {
        out.push_str(&e);
    }
    out
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
      Error::DbError(e) => async move { Err(Error::DbError(e)) }.boxed(),
      Error::RedbError(e) => async move { Err(Error::RedbError(e)) }.boxed(),
      Error::RedbTransactionError(e) => async move { Err(Error::RedbTransactionError(e)) }.boxed(),
      Error::RedbTableError(e) => async move { Err(Error::RedbTableError(e)) }.boxed(),
      Error::RedbStorageError(e) => async move { Err(Error::RedbStorageError(e)) }.boxed(),
      Error::RedbCommitError(e) => async move { Err(Error::RedbCommitError(e)) }.boxed(),
      Error::JsonError(e) => async move { Err(Error::JsonError(e)) }.boxed(),
    }
  }
}

///используется fold, оставить как памятку
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
