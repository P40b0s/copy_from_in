mod packet_table;
mod addresse_table;
use std::sync::Arc;
pub use addresse_table::AddresseTable;
use db_service::{SqlOperations, SqlitePool};
pub use packet_table::PacketTable;

pub async fn initialize_db(pool: Arc<SqlitePool>)
{
    let _ = PacketTable::create(Arc::clone(&pool)).await;
    let _ = AddresseTable::create(Arc::clone(&pool)).await;
}