use std::{path::Path, ffi::OsStr};

use serde::{Serialize, de::DeserializeOwned};



//#[serde(rename_all = "camelCase")]
pub trait FileMethods where Self : Clone + Serialize + Default + DeserializeOwned
{
    const FILE_NAME: &'static str;
    const FILE_PATH: &'static str;
    fn save(&self)
    {
        crate::io::serialize(self, Self::FILE_PATH, Self::FILE_NAME)
    }
    fn save_filename<P: AsRef<Path>>(&self, name: P)
    {
        crate::io::serialize(self, Self::FILE_PATH, name)
    }
    fn load() -> (bool, Self)
    {
        crate::io::deserialize(Self::FILE_PATH, Self::FILE_NAME)
    }
    fn load_file_name<P: AsRef<Path>>(name: P) -> (bool, Self)
    {
        crate::io::deserialize(Self::FILE_PATH, name)
    }
}