use std::fmt::Display;
use serde::{Serialize, Deserialize};

use super::Dictionary;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post 
{
    /**название должности*/
    post: Dictionary,
    /**принадлежность к отделу (1 отдел руководство итд...)*/
    department: Dictionary,
}