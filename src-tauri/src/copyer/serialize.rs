use std::{path::{PathBuf, Path}, fs::{OpenOptions, File}, io::{BufWriter, Write, Read}};
use logger::{error, warn};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Error;

///Сериализация объекта в строковый формат
pub fn serialize<T>(json : T, file_name : &Path, directory: Option<&str>) where T : Clone + Serialize
{
    let mut work_dir = PathBuf::default();
    if directory.is_some()
    {
        let p = PathBuf::from(directory.unwrap());
        work_dir = p;
        work_dir.push(file_name);
    }
    else
    {
        work_dir = std::env::current_dir().unwrap();
        work_dir.push(file_name);
    }
    //let _del = std::fs::remove_file(&work_dir);
    let write = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(work_dir);
    if write.is_err()
    {
        error!("{}", write.err().unwrap());
        return;
    }
    //let pretty = serde_json::to_string_pretty(&json);
    let ser = serde_json::to_string(&json);
    if let Ok(j) = ser
    {
        let mut f = BufWriter::new(write.unwrap());
        let _write = f.write_all(j.as_bytes());
    }
    else
    {
        error!("Ошибка сохранения файла настроек {}! -> {}", write.err().unwrap(), ser.err().unwrap());
        return;
    }
}


///Читение файл в строку из чистого utf-8
/// если false то файл не найден и был создан новый
pub fn deserialize<'de, T>(file_path: &Path) -> (bool, T) where T : Clone + DeserializeOwned + Default + Serialize
{
    let file = std::fs::read_to_string(file_path);
    if file.is_err()
    {
        let err = file.err().unwrap();
        warn!("Ошибка чтения файла {}, будет создан новый файл", file_path.display());
        return (false, T::default());
    }
    let result: Result<T, Error> = serde_json::from_str(&file.unwrap());
    if result.is_err()
    {
        //копируем бракованный файл чтобы можно было потом посмотреть в чем проблема  и скопировать оттуда настройки
        let err_settings = Path::new(file_path).join(".deserialization_error");
        std::fs::copy(file_path, &err_settings);
        error!("Ошибка десериализации файла {}->{}, будет создан новый файл c настройками по умолчанию", file_path.display(), result.err().unwrap());
        let default = T::default();

        serialize(default, file_path, None);
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