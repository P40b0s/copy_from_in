use std::fmt::Display;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::helpers::DaysProgress;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status
{
    pub id: String,
    /**id пользователя */
    pub user_id: String,
    pub start_date: String,
    pub end_date: String,
    pub place: String,
    pub days: DaysProgress,
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
    ///0 - отпуск 1 - командировка 2 - распоряжение
    pub status_type: u32
}



impl Status
{
    pub fn new(id: String, user_id: String, start_date: String, end_date: String, place: String, status_type: u32, note: Option<String>) -> Self
    {
        let days = DaysProgress::days_diff(&start_date, &end_date).unwrap_or_default();
        Status {
            id,
            user_id,
            start_date,
            end_date,
            place,
            days,
            note,
            status_type
        }
    }
}