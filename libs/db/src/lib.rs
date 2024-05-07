mod models;
mod tests;
mod error;
pub use error::DbError;
pub use models::{AddresseTable, PacketsTable, initialize_db};
//pub use rusqlite::{Connection, Result, Error};
pub use models::{Id, Operations, CountRequest, IdSelector, from_json, SortingOrder, Selector, QuerySelector};