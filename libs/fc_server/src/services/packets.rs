use std::path::Path;
use std::sync::Arc;
use logger::{debug, error};
use settings::{FileMethods, Task};
use transport::Packet;
use crate::copyer::ExcludesCreator;
use crate::state::AppState;
use crate::Error;

// pub async fn get(state: Arc<AppState>) -> Result<Vec<Packet>, Error>
// {
    
//     Ok(log)
// }
//TODO во время селекта надо вставлять туда таск по имени