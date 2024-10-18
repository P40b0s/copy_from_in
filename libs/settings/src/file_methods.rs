use serde::{Serialize, de::DeserializeOwned};
use crate::ValidationError;
pub use utilites::{Serializer, serialize, deserialize};

//#[serde(rename_all = "camelCase")]
pub trait FileMethods where Self : Clone + Serialize + Default + DeserializeOwned
{
    const FILE_PATH: &'static str;
    const PATH_IS_ABSOLUTE: bool;
    fn get_filename_with_extension(serializer: &Serializer) -> String
    {
        match serializer
        {
            Serializer::Json => [Self::FILE_PATH , ".json"].concat(),
            Serializer::Toml => [Self::FILE_PATH , ".toml"].concat()
        }
    }
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
    fn save(&self, serializer: Serializer) -> Result<(), Vec<ValidationError>>
    {
        let _val_ok = self.validate()?;
        let fp = Self::get_filename_with_extension(&serializer);
        
        if let Err(e) = serialize(self, &fp, Self::PATH_IS_ABSOLUTE, serializer)
        {
            Err(vec![ValidationError::new(None, e.to_string()); 1])
        }
        else 
        {
            Ok(())    
        }
    }
    
    fn load(serializer: Serializer) -> Result<Self, Vec<ValidationError>>
    {
        let des: Result<Self, utilites::error::Error> = deserialize(Self::FILE_PATH, Self::PATH_IS_ABSOLUTE, serializer);
        if let Ok(des) = des
        {
            des.validate()?;
            Ok(des)
        }
        else 
        {
            Err(vec![ValidationError::new_from_str(None, "Файл настроек не найден, создан новый файл, необходимо его правильно настроить до старта программы"); 1])
        }
    }
}