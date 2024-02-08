use std::path::Path;
use serde::{Serialize, de::DeserializeOwned};
use crate::ValidationError;

//#[serde(rename_all = "camelCase")]
pub trait FileMethods where Self : Clone + Serialize + Default + DeserializeOwned
{
    const FILE_NAME: &'static str;
    const FILE_PATH: &'static str;
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
    fn save(&self) -> Result<(), Vec<ValidationError>>
    {
        let _val_ok = self.validate()?;
        if let Err(e) = crate::io::serialize(self, Self::FILE_PATH, Self::FILE_NAME)
        {
            Err(vec![ValidationError::new(None, e); 1])
        }
        else 
        {
            Ok(())    
        }
    }
    fn save_filename<P: AsRef<Path>>(&self, name: P) -> Result<(), Vec<ValidationError>>
    {
        let _val_ok = self.validate()?;
        if let Err(e) = crate::io::serialize(self, Self::FILE_PATH, name)
        {
            Err(vec![ValidationError::new(None, e); 1])
        }
        else 
        {
            Ok(())    
        }
    }
    fn load() -> Result<Self, Vec<ValidationError>>
    {
        let des: (bool, Self) = crate::io::deserialize(Self::FILE_PATH, Self::FILE_NAME);
        if !des.0
        {
            Err(vec![ValidationError::new_from_str(None, "Файл настроек не найден, создан новый файл, необходимо его правильно настроить до старта программы"); 1])
        }
        else 
        {
            des.1.validate()?;
            Ok(des.1)
        }
    }
    fn load_file_name<P: AsRef<Path>>(name: P) -> (bool, Self)
    {
        crate::io::deserialize(Self::FILE_PATH, name)
    }
}