use std::sync::Arc;

use logger::error;
use serde::{Serialize, Deserialize};
use settings::{FileMethods, Settings, Task};
use uuid::Uuid;
use crate::copyer::PacketsCleaner;
use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::state::AppState;
use crate::Error;

pub async fn clear_dirs(state: Arc<AppState>) -> Result<u32, Error>
{
  let settings = state.get_settings().await;
  let r = Settings::clear_packets(&settings)?;
  Ok(r)
}

pub async fn truncate_tasks_excepts(state: Arc<AppState>) -> Result<u32, Error>
{
    let settings = state.get_settings().await;
    let r = settings.truncate_excludes();
    Ok(r)
}