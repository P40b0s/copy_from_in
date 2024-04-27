use std::fmt::Display;
use serde::{Deserialize, Serialize};
use settings::Task;

use crate::Packet;

impl service::Converter for Contract{}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Contract
{
    TaskUpdated(Task),
    TaskDeleted(Task),
    NewPacket(Packet),
    Error(String),
    ErrorConversion(String)
}



#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(untagged)]
pub enum ContractTest
{
    Test(String, u32, String),
    Test2 {subname: String, age: u32}
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestStr
{
    name: String,
    tst: ContractTest
}

#[cfg(test)]
mod tests
{
    //{"name":"3r2werwqe","tst":{"Test":["one",123,"three"]}}
    //{"name":"111111","tst":{"Test2":{"subname":"123321","age":123}}}
    use super::{ContractTest, TestStr};
    #[test]
    pub fn test()
    {
        let test = TestStr {
            name: "3r2werwqe".to_owned(),
            tst: ContractTest::Test("one".to_owned(),123,"three".to_owned())
        };
        let test2 = TestStr {
            name: "111111".to_owned(),
            tst: ContractTest::Test2 { subname: "123321".to_owned(), age: 123 }
        };
        let json = serde_json::to_string(&test).unwrap();
        println!("{}", json);
        let json = serde_json::to_string(&test2).unwrap();
        println!("{}", json);
    }
}