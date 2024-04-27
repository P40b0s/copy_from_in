
use crate::models::{AddresseTable, DbInterface, initialize_db};
use rusqlite::{Connection, Result};



#[cfg(test)]
mod tests 
{
    #[derive(Debug)]
    struct Person
    {
        id: i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    use std::time::UNIX_EPOCH;

    use rusqlite::{Connection, Result};
    use medo_parser::{PacketInfo, Requisites, MinistryOfJustice, SenderInfo};

    use crate::models::{AddresseTable, DbInterface, initialize_db, ContactInfo};

    #[test]
    fn test_addresse_create() -> Result<()>
    {
       AddresseTable::create()?;
       Ok(())
    }

    #[test]
    fn test_addresse_add() -> Result<()>
    {
       let addr = AddresseTable
       {
            id: Some(String::from("0b21bba1-f44d-4216-b465-147665360c06")),
            medo_addresse: Some(String::from("ADM_PREZ~MEDOGU")),
            organization: Some(String::from("Администрация Президента Российской Федерации")),
            notifications_sources_medo_addresses: vec![String::from("ADM_PREZ~MEDOGU"), String::from("ADM_PREZ2~MEDOGU")],
            icon: Some("base64".to_owned()),
            contact_info: vec![],
            update_key: None,
       };
       addr.add_or_replace()?;
       Ok(())
    }
    #[test]
    fn test_addresse_select() -> Result<()>
    {
        let addr = *AddresseTable::select("0b21bba1-f44d-4216-b465-147665360c06")?;
        assert_eq!(Some(String::from("ADM_PREZ~MEDOGU")), addr.medo_addresse);
        Ok(())
    }
    #[test]
    fn test_addresse_select_by_medo_addresse() -> Result<()>
    {
        let addr = *AddresseTable::select_by_medo_addr("ADM_PREZ~MEDOGU")?;
        assert_eq!(Some(String::from("ADM_PREZ~MEDOGU")), addr.medo_addresse);
        Ok(())
    }

    #[test]
    fn test_addresse_update() -> Result<()>
    {
        let mut addr = *AddresseTable::select("0b21bba1-f44d-4216-b465-147665360c06")?;
        addr.icon = Some("base64".to_owned());
        addr.update()?;
        let addr = *AddresseTable::select("0b21bba1-f44d-4216-b465-147665360c06")?;
        assert_eq!(Some("base64".to_owned()), addr.icon);
        Ok(())
    }

    #[test]
    fn test_icon_update() -> Result<()>
    {
        logger::StructLogger::initialize_logger();
        let mut addr = *AddresseTable::select("0b21bba1-f44d-4216-b465-147665360c06")?;
        //addr.change_icon("/home/phobos/Изображения/prez.jpg")?;
        let addr = *AddresseTable::select("0b21bba1-f44d-4216-b465-147665360c06")?;
        logger::debug!("Заменена иконка {:?}", addr.icon);
        Ok(())
    }

    #[test]
    fn test_addresse_delete() -> Result<()>
    {
       let addr = AddresseTable
       {
            id: Some(String::from("0b21bba1-f44d-4216-b465-147665360c06")),
            medo_addresse: Some(String::from("ADM_PREZ~MEDOGU")),
            organization: Some(String::from("Администрация Президента Российской Федерации")),
            notifications_sources_medo_addresses: vec![String::from("ADM_PREZ~MEDOGU"), String::from("ADM_PREZ2~MEDOGU")],
            icon: Some("base64".to_owned()),
            contact_info: vec![],
            update_key: None,
       };
       addr.delete()?;
       Ok(())
    }


    #[test]
    fn test_drop_table_addresses() -> Result<()>
    {
       AddresseTable::drop(false)?;
       Ok(())
    }
    #[test]
    fn test_drop_table_packets() -> Result<()>
    {
       PacketInfo::drop(false)?;
       Ok(())
    }


    #[test]
    fn test_packet_add() -> Result<()>
    {
        logger::StructLogger::initialize_logger();
        initialize_db();
        let pi = PacketInfo
        {
                packet_directory: "54139378".to_owned(),
                pdf_hash: None,
                acknowledgment: None,
                visible: true,
                header_guid: Some("511a7c7a-7503-4ea4-a195-9fb6287c7818".to_owned()),
                packet_type: Some("Документ".to_owned()),
                delivery_time: "2023-02-14T13:22:24".to_owned(),
                update_key: "2023-02-14T13:22:24".to_owned(),
                wrong_encoding: false,
                error: None,
                trace_message: None,
                files: vec![ "76608808-76609069.pdf".to_owned(),
                "envelope.ltr".to_owned(),
                "document.xml".to_owned()],
                requisites: Some(Requisites
                {
                    document_guid: Some("494b76c6-89dd-4a5f-90a7-c39b60b7ce84".to_owned()),
                    act_type: Some("Зарегистрированные НПА ФОИВ".to_owned()),
                    document_number: Some("130/23-НПА".to_owned()),
                    sign_date: Some("2023-01-23".to_owned()),
                    pages: Some(8),
                    annotation: Some("72097 от 23.01.2023".to_owned()),
                    mj: Some(MinistryOfJustice
                    {
                        number: "72097".to_owned(),
                        date: "2023-01-23".to_owned()
                    })
                }),
                sender_info: Some(SenderInfo
                {
                    organization: Some("Минюст России".to_owned()),
                    person: Some("Государственная регистрация".to_owned()),
                    department: Some("ДВА (01) Департамент государственной регистрации ведомственных нормативных правовых актов".to_owned()),
                    post: Some("Минюст России".to_owned()),
                    medo_addessee: Some("M_MJUST_S~MEDOGU".to_owned()),
                    addressee: None,
                    source_guid: Some("7c960fc6-2745-4655-80eb-fed1a7905325".to_owned()),
                    executor: Some(medo_parser::Executor 
                    { 
                        organization: Some("Минюст России".to_owned()),
                        person: Some("Государственная регистрация".to_owned()),
                        post: Some("Минюст России".to_owned()),
                        contact_info: None
                    })
                }),
                default_pdf: Some("76608808-76609069.pdf".to_owned())
        };

        pi.add_or_replace()?;
        
        Ok(())
    }

    #[test]
    fn test_packet_delete() -> Result<()>
    {
        logger::StructLogger::initialize_logger();
        //наконец то ищет
        let mj_search = PacketInfo::json_query("requisites->'mj'->'number' = '\"72097\"'");
        let packet = *PacketInfo::select("511a7c7a-7503-4ea4-a195-9fb6287c7818")?;
        let pp = serde_json::to_string_pretty(&packet).unwrap();
        Ok(())
    }

    #[test]
    fn test_packet_select() -> Result<()>
    {
        let addr = *PacketInfo::select("df733c70-9b8f-418d-b4b4-7d92597d4cfc")?;
        assert_eq!(String::from("2023-02-14T13:28:39"), addr.delivery_time);
        Ok(())
    }
    #[test]
    fn test_query_select() -> Result<()>
    {
        let a = PacketInfo::query(None, [])?;
        Ok(())
    }
    

    #[test]
    fn test_time()
    {
        logger::StructLogger::initialize_logger();
        let t = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
        println!("{:?}", t);
      
    }
}