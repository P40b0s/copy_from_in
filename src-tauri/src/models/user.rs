use std::fmt::Display;
use serde::{Serialize, Deserialize};
use super::{Dictionary, Disease, DiseaseTest, Phones, Post, Status, Vactination};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User 
{
    pub id: String,
    pub name1: String,
    pub name2: String,
    pub surname: String,
    pub post: Dictionary,
    pub department: Dictionary,
    pub san_ticket_number: String,
    pub bornsday: String,
    pub rank: Dictionary,
    /**место жительства */
    pub live_place: String,
    pub phones: Vec<Phones>,
    /**тесты на заболевание (фактически только для ковид но дальше хз) */
    pub tests: Vec<DiseaseTest>,
    pub diseases: Vec<Disease>,
    pub statuses: Vec<Status>,
}

impl User
{
    pub fn status_count(&self, status: u32) -> u32
    {
        self.statuses.iter().filter(|f| f.status_type == status).collect::<Vec<&Status>>().len() as u32
    }
    pub fn surname_with_initials(&self) -> String
    {
        [&self.surname, " ", &self.name1[..2], ".", &self.name2[..2], "."].concat()
    }
    pub fn full_name(&self) -> String
    {
        [&self.surname, " ", &self.name1, ".", &self.name2, "."].concat()
    }
}

#[cfg(test)]
mod tests
{
    // #[test]
    // fn test_fio()
    // {
    //     let f = super::Fio::new("Иксар Алексей Игоревич");
    //     println!("{}, {}", &f, f.surname_with_initials());

    // }
}