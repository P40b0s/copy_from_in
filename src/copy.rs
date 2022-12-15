use std::{self, path::{Path}};
use crate::settings::Settings;

pub fn get_except_names(settings: &Settings) -> Option<Vec<String>>
{
    if let Some(dirs) = get_dirs(settings)
    {
        // if std::fs::write("except.dirs", dirs.join("\n")).is_err()
        // {
        //     eprintln!("😳 Немогу создать файл искоючений except.dirs!");
        //     return None;
        // }
        return  Some(dirs);
    }
    else {
        return None;
    }
}

pub fn get_dirs(settings: &Settings) -> Option<Vec<String>>
{
    let paths = std::fs::read_dir(settings.in_dir.as_path());
    if paths.is_err()
    {
        eprintln!("😳 Ошибка чтения директории {} - {}", settings.in_dir.display(), paths.err().unwrap());
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

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> std::io::Result<()> 
{
    std::fs::create_dir_all(&destination)?;
    for entry in std::fs::read_dir(source)? 
    {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() 
        {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } 
        else 
        {
            std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
