mod settings;
mod packets;
mod date;
mod websocket;
pub use websocket::WebsocketPlugin;
pub use date::DatePlugin;
use std::sync::Arc;
pub use packets::PacketsPlugin;
pub use settings::SettingsPlugin;
mod helpers;
mod service;
pub use service::ServicePlugin;
use tauri::{plugin::TauriPlugin, Runtime};
use crate::state::AppState;


pub trait Plugin
{
    ///Имя плагина
    const NAME: &str;
    fn build<R: Runtime>(app_state: Arc<AppState>) -> TauriPlugin<R>;
}
pub trait PluginContext
{
    ///Имя плагина
    const NAME: &str;
    fn build<R: Runtime>(&self, app_state: Arc<AppState>) -> TauriPlugin<R>;
}