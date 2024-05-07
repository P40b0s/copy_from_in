use std::path::Path;
//use rusqlite::{Connection, Result};
use anyhow::{anyhow, Result};
use sqlx::{Connection, SqliteConnection};

// pub fn get_connection() -> Result<Connection>
// {
//     let local_path = Path::new(&std::env::current_dir().unwrap()).join("db.sql");
//     let conn = Connection::open(local_path.clone())?;
//     Ok(conn)
// }

pub async fn get_connection() -> Result<SqliteConnection> 
{
    let local_path = Path::new(&std::env::current_dir().unwrap()).join("data.sql");
    if !local_path.exists()
    {
        std::fs::File::create(&local_path).map_err(|_| anyhow!("Ошибка создания файла базы данных!"))?;
    }
    Ok(SqliteConnection::connect(&local_path.display().to_string()).await?)
}
// pub fn get_connection() -> Result<Connection>
// {
//     let local_path = Path::new(&std::env::current_dir().unwrap()).join("data.sql");
//     //logger::debug!("База данных указаная в настройках программы {} не найдена, будет создана новая база данных в директории исполняемой программы {}", &settings_db_path.display(), &local_path.display());
//     let conn = Connection::open(local_path.clone())?;
//     // if let Some(lp) = local_path.to_str()
//     // {
//     //     SETTINGS.write().unwrap().paths.db_file_path = lp.to_owned();
//     // }
//     Ok(conn)
// }