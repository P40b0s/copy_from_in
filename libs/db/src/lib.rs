mod models;
mod tests;
mod error;
pub use error::DbError;
pub use models::{AddresseTable, DbInterface, initialize_db, BatchOperation, PacketTypesCount};
pub use rusqlite::{Connection, Result, Error};