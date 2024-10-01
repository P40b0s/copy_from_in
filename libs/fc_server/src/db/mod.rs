mod packet_table;
mod addresse_table;
mod contact_info;

use std::sync::Arc;

pub use addresse_table::AddresseTable;
use db_service::{SqlOperations, SqlitePool};
pub use packet_table::PacketTable;

pub async fn initialize_db(pool: Arc<SqlitePool>)
{
    let _ = packet_table::PacketTable::create(Arc::clone(&pool)).await;
    let _ = addresse_table::AddresseTable::create(Arc::clone(&pool)).await;
}