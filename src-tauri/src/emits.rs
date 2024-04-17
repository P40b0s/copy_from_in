use settings::Task;
use transport::NewPacketInfo;

use crate::HANDLE;

///Эмиты из бэка во фронтенд
pub struct TauriEmits;
impl TauriEmits
{
    pub fn packets_update(packet: NewPacketInfo)
    {
        HANDLE.get().unwrap().app_handle().emit_all("packets_update", packet);
    }
    pub fn error(error: String)
    {
        HANDLE.get().unwrap().app_handle().emit_all("error", error);
    }
    pub fn task_updated(task: Task)
    {
        HANDLE.get().unwrap().app_handle().emit_all("task_updated", task);
    }
    pub fn task_deleted(task: Task)
    {
        HANDLE.get().unwrap().app_handle().emit_all("task_deleted", task);
    }
}