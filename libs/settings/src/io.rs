use std::{path::{PathBuf, Path}, fs::{OpenOptions, File}, io::{BufWriter, Write, Read}};
use logger::{error, warn};
use serde::{Serialize,  de::DeserializeOwned};

#[derive(Clone)]
pub enum Serializer
{
    Json,
    Toml
}
///Сериализация объекта в строковый формат
///если linux то 
pub fn serialize<T, P: AsRef<Path>>(json : T, file_path : P, path_as_absolute: bool, serializer: Serializer) -> Result<(), String> where T : Clone + Serialize 
{
    let path = if !path_as_absolute
    {
        Path::new(&std::env::current_dir().unwrap()).join(file_path)
    }
    else
    {
        file_path.as_ref().to_path_buf()
    };
   
    let write = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(&path);

    if let Ok(wr) = write
    {
        //let pretty = serde_json::to_string_pretty(&json);
        let ser = match serializer
        {
            Serializer::Toml => toml::to_string(&json).map_err(|e| e.to_string()),
            Serializer::Json => serde_json::to_string_pretty(&json).map_err(|e|e.to_string())
        };
     
        if let Ok(toml) = ser
        {
            let mut f = BufWriter::new(wr);
            let _write = f.write_all(toml.as_bytes());
            return Ok(());
        }
        else
        {
            let err = ["Ошибка сохранения файла настроек ", &path.display().to_string(), " -> ", &ser.err().unwrap().to_string()].concat();
            error!("{}", &err);
            return Err(err);
        }
    }
    else 
    {
        let err = ["Ошибка сохранения файла настроек -> ", &write.err().unwrap().to_string()].concat();
        error!("{}", &err);
        return Err(err);
    }
   
}


///Читение файл в строку из чистого utf-8
/// если false то файл не найден и был создан новый
pub fn deserialize<'de, T, P: AsRef<Path>>(file_path: P, path_as_absolute: bool, serializer: Serializer) -> (bool, T) where T : Clone + DeserializeOwned + Default
{
    let path = if !path_as_absolute
    {
        Path::new(&std::env::current_dir().unwrap()).join(file_path)
    }
    else
    {
        file_path.as_ref().to_path_buf()
    };
    let file = std::fs::read_to_string(&path);
    if file.is_err()
    { 
        
        let err = file.err().unwrap();
        warn!("Ошибка чтения файла {}, текущий объект инициализирован с настроками по умолчанию {}", &path.display(), err);
        return (false, T::default());
    }
    //let result: Result<T, Error> = toml::from_str(&file.unwrap());
    let result: Result<T, String>  = match serializer
    {
        Serializer::Toml => toml::from_str(file.as_ref().unwrap()).map_err(|e| e.to_string()),
        Serializer::Json => serde_json::from_str(file.as_ref().unwrap()).map_err(|e|e.to_string())
    };
    if result.is_err()
    {
        let err_settings = Path::new(&path).join(".structure_error");
        let _ = std::fs::copy(&path, &err_settings);
        error!("Ошибка десериализации файла {}->{}, текущий объект инициализирован с настроками по умолчанию", &path.display(), result.err().unwrap());
        return (false, T::default());
    }
    return (true, result.unwrap());
}

///Чтение файла в бинарный формат
pub fn read_file_to_binary(file_path: &PathBuf) -> Option<Vec<u8>>
{
    if let  Ok(f) = File::open(file_path).as_mut()
    {
        //f.read(&mut buffer);
        let mut buffer = Vec::new();
        if f.read_to_end(&mut buffer).is_ok()
        {
            return Some(buffer);
        }
        else 
        {
            return None;
        }
    }
    None
}

pub fn get_dirs(path: &PathBuf) -> Option<Vec<String>>
{
    let paths = std::fs::read_dir(path);
    if paths.is_err()
    {
        error!("😳 Ошибка чтения директории {} - {}", path.display(), paths.err().unwrap());
        return None;
    }
    let mut dirs = vec![];
    for d in paths.unwrap()
    {
        let dir = d.unwrap().file_name().to_str().unwrap().to_owned();
        dirs.push(dir);
    }
    return Some(dirs);
}