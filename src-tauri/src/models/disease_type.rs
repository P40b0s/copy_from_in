use std::fmt::Display;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiseaseType
{
    pub id: String,
    pub name: String,
    pub need_reference: bool
}
impl Display for DiseaseType 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        f.write_str(&self.name)
        
    }
}


#[cfg(test)]
mod tests
{
    use uuid::Uuid;

  
}