use serde::{Serialize, de::DeserializeOwned};
use crate::ValidationError;

//#[serde(rename_all = "camelCase")]
pub trait FileMethods where Self : Clone + Serialize + Default + DeserializeOwned
{
    const FILE_PATH: &'static str;
    const PATH_IS_ABSOLUTE: bool;
    fn get_filename_with_extension(serializer: &super::io::Serializer) -> String
    {
        match serializer
        {
            super::io::Serializer::Json => [Self::FILE_PATH , ".json"].concat(),
            super::io::Serializer::Toml => [Self::FILE_PATH , ".toml"].concat()
        }
    }
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
    fn save(&self, serializer: super::io::Serializer) -> Result<(), Vec<ValidationError>>
    {
        let _val_ok = self.validate()?;
        let fp = Self::get_filename_with_extension(&serializer);
        if let Err(e) = crate::io::serialize(self, &fp, Self::PATH_IS_ABSOLUTE, serializer)
        {
            Err(vec![ValidationError::new(None, e); 1])
        }
        else 
        {
            Ok(())    
        }
    }
    // fn save_filename<P: AsRef<Path>>(&self, name: P) -> Result<(), Vec<ValidationError>>
    // {
    //     let _val_ok = self.validate()?;
    //     if let Err(e) = crate::io::serialize(self, Self::FILE_PATH, name)
    //     {
    //         Err(vec![ValidationError::new(None, e); 1])
    //     }
    //     else 
    //     {
    //         Ok(())    
    //     }
    // }
    fn load(serializer: super::io::Serializer) -> Result<Self, Vec<ValidationError>>
    {
        let des: (bool, Self) = crate::io::deserialize(Self::FILE_PATH, Self::PATH_IS_ABSOLUTE, serializer);
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
    // fn load_file_name<P: AsRef<Path>>(name: P) -> (bool, Self)
    // {
    //     crate::io::deserialize(Self::FILE_PATH, name)
    // }
}