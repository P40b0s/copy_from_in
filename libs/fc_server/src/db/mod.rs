mod packet_table;
mod addresse_table;
mod contact_info;

pub use addresse_table::AddresseTable;
pub use packet_table::PacketTable;
pub use db_service::{Operations, Selector};

pub async fn initialize_db()
{
    packet_table::PacketTable::create().await;
    addresse_table::AddresseTable::create().await;
}