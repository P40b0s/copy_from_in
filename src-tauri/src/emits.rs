use settings::Task;
use transport::Packet;
use tauri::Manager;

use crate::HANDLE;

///Эмиты из бэка во фронтенд
pub struct TauriEmits;
impl TauriEmits
{
    pub fn packets_update(packet: Packet)
    {
        let _ = HANDLE.get().unwrap().app_handle().emit_all("packets_update", packet);
    }
    pub fn error(error: String)
    {
        let _ = HANDLE.get().unwrap().app_handle().emit_all("error", error);
    }
    pub fn task_updated(task: Task)
    {
        let _ = HANDLE.get().unwrap().emit_all("task_updated", task);
    }
    pub fn task_deleted(task_name: String)
    {
        let _ = HANDLE.get().unwrap().app_handle().emit_all("task_deleted", task_name);
    }
    pub fn clean_start()
    {
        let _ = HANDLE.get().unwrap().app_handle().emit_all("clean_start", ());
    }
    pub fn clean_complete(count: u32)
    {
        let _ = HANDLE.get().unwrap().app_handle().emit_all("clean_complete", count);
    }
}