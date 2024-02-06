use std::sync::Arc;

use serde::Serialize;
use tauri::Manager;

use crate::{state::AppState, helpers::{Date, DateTimeFormat, DateFormat, DaysProgress}, TauriEvent, models::User};

pub trait StateUpdater where Self: Sized + Serialize + Clone
{
    async fn update_from_thread<R: tauri::Runtime>(manager: Arc<impl Manager<R>>) -> anyhow::Result<()>
    {
        let s = manager.state::<AppState>().inner();
        let updater = Self::update(s).await?;
        let _ = manager.emit_all(&TauriEvent::UpdateState.to_string(), updater);
        Ok(())
    }
    async fn update_from_command(state: tauri::State<'_, AppState>) -> anyhow::Result<Self>
    {
        let s = state.inner();
        let updater = Self::update(s).await?;
        Ok(updater)
    }
    async fn update(state: &AppState) -> anyhow::Result<Self>;
}

impl StateUpdater for DateState
{
    async fn update_from_thread<R: tauri::Runtime>(manager: Arc<impl Manager<R>>) -> anyhow::Result<()>
    {
        let s = manager.state::<AppState>().inner();
        let updater = Self::update(s).await?;
        let _ = manager.emit_all(&TauriEvent::UpdateDate.to_string(), updater);
        Ok(())
    }
    async fn update(state: &AppState) -> anyhow::Result<Self>
    {
        let updater: Self = state.into();
        Ok(updater)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DateState
{
    pub current_date: String
}

#[derive(Debug, Clone, Serialize)]
pub struct LogState
{
    pub errors: Vec<String>,
    pub log: Vec<String>,
}

// impl FrontendStateUpdater
// {
//     pub async fn update_from_thread<R: tauri::Runtime>(manager: Arc<impl Manager<R>>) -> anyhow::Result<()>
//     {
//         let s = manager.state::<AppState>().inner();
//         let updater = Self::update(s).await?;
//         let _ = manager.emit_all(&TauriEvent::UpdateState.to_string(), updater);
//         Ok(())
//     }
//     pub async fn update_from_command(state: tauri::State<'_, AppState>) -> anyhow::Result<Self>
//     {
//         let s = state.inner();
//         let updater = Self::update(s).await?;
//         Ok(updater)
//     }
//     async fn update(state: &AppState) -> anyhow::Result<Self>
//     {
//         let updater: Self = Self {
//             current_disease_users: vec![],
//             users_with_statuses: vec![]
//         };
//         //logger::info!("date:{} dis:{} vac:{} dis_users_len:{}, ord: {}, users_count:{}", &updater.current_date, &updater.diseases_count, &updater.vacations_count, &updater.current_disease_users.len(), &updater.ordered_count, &updater.users_count);
//         Ok(updater)
//     }

// }