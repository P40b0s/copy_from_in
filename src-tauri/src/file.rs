use std::{path::{PathBuf, Path}, fs::{OpenOptions, File}, io::{BufWriter, BufReader, Write}, error::Error};
use logger::error;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use crate::models::*;

pub static USERS: Lazy<Vec<User>> = Lazy::new(|| *Vec::<User>::load("users.json").unwrap());



// .map(line_processor)
//         .sorted_by(|h1, h2| h2.strength.cmp(&h1.strength))
//         .enumerate() // <- after that do some println
//         .map(|(idx, hand)| Hand {rank: (idx + 1) as i32, ..hand})


pub trait Save
{
	///Сериализация объекта в строковый формат
	fn save<T>(json : T, file_name : &str, directory: Option<&str>) where T : Clone + Serialize
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
		let write = OpenOptions::new()
		.write(true)
		.create(true)
		.open(work_dir);
		if write.is_err()
		{
			error!("{}", write.err().unwrap());
			return;
		}
		let pretty = serde_json::to_string_pretty(&json);
		if let Ok(pretty) = pretty
		{
			let mut f = BufWriter::new(write.unwrap());
			let _write = f.write_all(pretty.as_bytes());
		}
		else
		{
			error!("Ошибка десериализации файла {} -> {}", write.err().unwrap(), pretty.err().unwrap());
			return;
		}
		// work_dir.push(file_name);
		// let path = std::path::Path::new(&work_dir);
		// let mut file = std::fs::File::create(&path).unwrap();
		// file.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes()).unwrap();
		// file.flush().unwrap();
			
	}
}

pub trait Deserializer
{
	fn load<P: AsRef<Path>>(path: P) -> Result<Box<Self>, Box<dyn Error>>;
}

impl Deserializer for User
{
	fn load<P: AsRef<Path>>(path: P) -> Result<Box<Self>, Box<dyn Error>>
	{
		let path = Path::new(path.as_ref());
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let des = serde_json::from_reader(reader)?;
		Ok(Box::new(des)) 
	}
}

impl Deserializer for Vec<User>
{
	fn load<P: AsRef<Path>>(path: P) -> Result<Box<Self>, Box<dyn Error>>
	{
		let path = Path::new(path.as_ref());
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let des = serde_json::from_reader(reader)?;
		Ok(Box::new(des)) 
	}
}


#[cfg(test)]
mod tests
{
    #[test]
    fn test_fio()
    {
        
        

    }
}