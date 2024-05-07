
mod packet;
pub use packet::PacketsTable;
mod connection;
mod contact_info;
mod operations;
pub use operations::{Id, Operations, CountRequest, IdSelector, from_json, SortingOrder, Selector, QuerySelector};
pub use contact_info::ContactInfo;
mod addresse;
pub use addresse::AddresseTable;
use std::slice::Iter;

use connection::get_connection;
use medo_parser::{Requisites, SenderInfo, PacketInfo};

///Создание если не существует база данных
pub async fn initialize_db()
{
    let _cr1 = AddresseTable::create().await;
    let _cr2 = PacketsTable::create().await;
    let r = "";
}
//Трейт для работы со структурой как с базой данных
// pub trait DbInterface
// {
//     const SELECT_BODY: &'static str;
//     ///where_params вписываем параметры для поиска, например delivery_time = :delivery_time and packet_type = :packet_type
//     /// а параметры это уже передача значений, передаются в таком виде - &[(":delivery_time", "2020-20-20"), (":packet_type", "Документ")]
//     /// полный пример параметров: (Some("header_id = :header_id"), &[(":header_id", id)])
//     fn query<P>(where_params: Option<&str>, params: P) -> Result<Vec<Box<Self>>> where P : Params 
//     {
//         let c = get_connection()?;
//         let create_body = match where_params
//         {
//             Some(w) => [Self::SELECT_BODY, " WHERE ", w].concat(),
//             None => Self::SELECT_BODY.to_owned()
//         };
//         let mut stmt = c.prepare(&create_body)?;
//         let mut rows = stmt.query(params)?;
//         Self::select_body_query(&mut rows)
//     }

//     ///Пример параметра json запроса: "requisites->'mj'->'number' = '\"72097\"'"
//     fn json_query(where_params: &str) -> Result<Vec<Box<Self>>>
//     {
//         let c = get_connection()?;
//         let create_body = [Self::SELECT_BODY, " WHERE ", where_params].concat();
//         let mut stmt = c.prepare(&create_body)?;
//         let mut rows = stmt.query([])?;
//         Self::select_body_query(&mut rows)
//     }
//     ///Пример параметра json запроса: "requisites->'mj'->'number' = '\"72097\"'"
//     fn custom_query<F, T, P : Params >(query_body: &str, f: F, params: P) -> Result<Vec<T>> where F: Fn(&mut Rows) -> Result<Vec<T>>
//     {
//         let c = get_connection()?;
//         let mut stmt = c.prepare(&query_body)?;
//         let mut rows = stmt.query(params)?;
//         let serialize = f(&mut rows)?;
//         Ok(serialize)
//         //Self::select_body_query(&mut rows)
//     }
//     //fn query<P>(where_params: &str, params: P) -> Result<Vec<Box<Self>>> where P : Params;
//     fn select_body_query(rows: &mut Rows) -> Result<Vec<Box<Self>>>;
//     ///Создать базу данных данной структуры
//     fn create() -> Result<()>;
//     ///Добавить, если есть заменить указанный обьект в базу данных
//     fn add_or_replace(&self) -> Result<()>;
//     ///Добавить, если есть игнорировать указанный обьект в базу данных
//     fn add_or_ignore(&self) -> Result<()>;
//     ///удалить указанный обьект из базы данных
//     ///если установлен флаг all то будет произведено полное удаление всех записей
//     fn delete(&self) -> Result<()>;
//     ///Обновить обьект в базе даных (по первичному ключу)
//     fn update(&self) -> Result<()>;
//     ///Выбор обьекта из базы данных по первичному ключу
//     fn select(id: &str) -> Result<Box<Self>>;
//     fn drop(clean: bool) -> Result<()>;
   
// }

// pub trait BatchOperation
// {
//     ///Добавление  в базу данных массива данных в пределах одной транзакции
//     fn batch(&self) -> Result<()>  where
//     Self: Default + Sized;
// }

// pub trait DbSelectionInterface
// {
//     const BODY: &'static str;
//     ///where_params вписываем параметры для поиска, например delivery_time = :delivery_time and packet_type = :packet_type
//     /// а параметры это уже передача значений, передаются в таком виде - &[(":delivery_time", "2020-20-20"), (":packet_type", "Документ")]
//     fn query<P>(where_params: &str, params: P) -> Result<Vec<Box<PacketInfo>>> where P : Params;
// }