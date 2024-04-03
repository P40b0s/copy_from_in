use std::{fmt::Display, path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
//#[serde(rename_all = "camelCase")]
pub struct Task
{
    pub name: String,
    #[serde(default="def_str")]
    pub description: String,
    #[serde(default="def_dirs")]
    pub source_dir: PathBuf,
    #[serde(default="def_dirs")]
    pub target_dir: PathBuf,
    #[serde(default="def_dirs")]
    pub report_dir: PathBuf,
    #[serde(default="def_timer")]
    pub timer: u64,
    #[serde(default="is_default")]
    pub delete_after_copy: bool,
    //#[serde(default="def_copy_mod")]
    //#[serde(deserialize_with="deserialize_copy_modifier")]
    pub copy_modifier: CopyModifier,
    #[serde(default="is_default")]
    pub is_active: bool,
    ///Типы пакетов которые будут очищаться
    #[serde(default="empty_doc_types")]
    pub clean_types: Vec<String>,
    #[serde(default="is_default")]
    pub generate_exclude_file: bool,
    #[serde(default="def_col")]
    pub color: String,
    pub filters: Filter
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
//#[serde(rename_all = "camelCase")]
pub struct Filter
{
    #[serde(default="empty_doc_types")]
    pub document_types: Vec<String>,
    #[serde(default="empty_doc_types")]
    pub document_uids: Vec<String>
}

fn is_default() -> bool
{
    false
}
fn def_timer() -> u64
{
    200000
}
fn def_copy_mod() -> CopyModifier
{
    CopyModifier::CopyAll
}
fn empty_doc_types() -> Vec<String>
{
    Vec::with_capacity(0)
}
fn def_dirs() -> PathBuf
{
    PathBuf::from("---")
}
fn def_str() -> String
{
    "".to_owned()
}
fn def_col() -> String
{
    "#4f46".to_owned()
}

impl Default for Task
{
    fn default() -> Self 
    {
        Task
        {
            source_dir: PathBuf::from("in"),
            target_dir: PathBuf::from("out"),
            report_dir: PathBuf::from(""),
            timer: 20000,
            name: "default_task".to_owned(),
            description: "".to_owned(),
            copy_modifier: CopyModifier::CopyAll,
            delete_after_copy: false,
            is_active: false,
            clean_types: vec![],
            generate_exclude_file: false,
            color: def_col(),
            filters: Filter
            {
                document_types: vec![],
                document_uids: vec![]
            }
            
        }
    }
}

impl Task
{
    pub fn get_task_name(&self) -> &str
    {
        &self.name
    }
    pub fn get_source_dir(&self) -> &PathBuf
    {
        &self.source_dir
    }
    pub fn get_target_dir(&self) -> &PathBuf
    {
        &self.target_dir
    }
    pub fn get_task_delay(&self) -> Duration
    {
        std::time::Duration::from_millis(self.timer)
    }
    fn have_report_dir(&self) -> bool
    {
        self.report_dir.to_str().is_some_and(|r| r != "")
    }
    pub fn get_report_dir(&self) -> Option<&PathBuf>
    {
        if self.have_report_dir()
        {
            Some(&self.report_dir)
        }
        else
        {
            None
        }
    }
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CopyModifier
{
    CopyAll,
    CopyOnly,
    CopyExcept
}
impl Display for CopyModifier
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", match self 
        {
            CopyModifier::CopyAll => "CopyAll",
            CopyModifier::CopyOnly => "CopyOnly",
            CopyModifier::CopyExcept => "CopyExcept"
        })
    }
}


fn deserialize_copy_modifier2<'de, D>(deserializer: D) -> Result<CopyModifier, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    match s.as_str()
    {
        "CopyOnly" => Ok(CopyModifier::CopyOnly),
        "CopyAll" => Ok(CopyModifier::CopyAll),
        "CopyExcept" => Ok(CopyModifier::CopyExcept),
        _ => Err(serde::de::Error::custom("Модификатор может быть только: CopyOnly, CopyAll, CopyExcept"))
    }
}

// fn deserialize_copy_modifier<'de, D>(deserializer: D) -> Result<CopyModifier, D::Error>
// where
//     D: serde::de::Deserializer<'de>,
// {
//     struct CopyModiferVisitor;

//     impl<'de> serde::de::Visitor<'de> for CopyModiferVisitor {
//         type Value = CopyModifier;
    
//         fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//             formatter.write_str("Модификатор копирования может быть только: copy_only, copy_all, copy_except")
//         }
    
//         fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//         where
//             E: serde::de::Error,
//         {
//             // unfortunately we lose some typed information
//             // from errors deserializing the json string
//             match v
//             {
//                 "copy_only" => Ok(CopyModifier::CopyOnly),
//                 "copy_all" => Ok(CopyModifier::CopyAll),
//                 "copy_except" => Ok(CopyModifier::CopyExcept),
//                 _ => Err(serde::de::Error::custom("Модификатор может быть только: copy_only, copy_all, copy_except"))
//             }
//             //serde_json::from_str(v).map_err(E::custom)
//         }
//     }
    
//     // use our visitor to deserialize an `ActualValue`
//     deserializer.deserialize_any(CopyModiferVisitor)
// }
// impl<'de> serde::Deserialize<'de> for CopyModifier
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: serde::Deserializer<'de> 
//     {
//         let s: String = serde::de::Deserialize::deserialize(deserializer)?;
//         match s.as_str()
//         {
//             "copy_only" => Ok(CopyModifier::CopyOnly),
//             "copy_all" => Ok(CopyModifier::CopyAll),
//             "copy_except" => Ok(CopyModifier::CopyExcept),
//             _ => Err(serde::de::Error::custom("Модификатор может быть только: copy_only, copy_all, copy_except"))
//         }
//     }
// }

// impl serde::Serialize for CopyModifier
// {
//     fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//        self.to_string().serialize(s)
//     }
// }