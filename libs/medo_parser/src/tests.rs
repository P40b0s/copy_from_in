
use std::{borrow::Borrow, io::{BufWriter, Write, Read, BufReader}, fs::{OpenOptions, FileType, DirEntry, File}, path::{Path, PathBuf}, time::Instant, str::from_utf8};
use crate::{medo_model::PacketInfo, DatesHelper, Ltr, MedoParser, Packet};

use super::{RootContainer,Container, Communication};
use logger::{debug, error, info};
use quick_xml::DeError;
//use quickxml_to_serde::{Config, xml_string_to_json, JsonArray, JsonType};
use serde_json::{json, Value};
use regex;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_1251;
use utilites::Date;
use super::io::*;
use crate::helpers;
// const LTR: &str = r#"[ПИСЬМО КП ПС СЗИ]
// ТЕМА=ЭСД МЭДО (78 от 25.01.2023 {9F50BC3D-47AC-4446-B528-678BB8FB0C30})
// АВТООТПРАВКА=1
// ЭЦП=0
// ДОСТАВЛЕНО=1
// ПРОЧТЕНО=1
// ДАТА=26.01.2023 13:22:52



// [АДРЕСАТЫ]
// 0=WH-MEDO~APRF
// [ФАЙЛЫ]
// 0=text0000000000.pdf
// 1=document.xml
// [ТЕКСТ]
// ФАЙЛ=annotation.txt
// "#;

const ABSOLUTE_PATH : &str = "/hard/xar/medo_testdata/0";
#[test]
fn test_ltr()
{
    logger::StructLogger::new_default();
    let eror_enc = Path::new(ABSOLUTE_PATH)
    .join("ltrs")
    .join("error_encoding.ltr");
    let d = super::open_file(&eror_enc, Some(FileEncoding::Windows1251));
    assert_eq!(d.is_err(), true);
    let default = Path::new(ABSOLUTE_PATH)
    .join("ltrs")
    .join("default.ltr");
    let date = "23.01.2023 09:56:17";
    let addr = "M_MID_S~MEDOGU";
    let file_1 = "62b466c9_fb5d_7d9f_42e0_455173b29091.pdf";
    let file_2 = "document.xml";
    let theme = "ЭСД МЭДО (МД-1654 от 2022-10-29 {e4595e23-16f9-4a17-ae0b-e2a2385640d7})";
    if let Ok(decoded) = super::open_file(&default, None)
    {
        let ltr = Ltr::parse(&default).unwrap();
        
        //write_string_to_file(&decoded, "ltr_file");
        assert_eq!(ltr.addresses.iter().nth(0).unwrap(), addr);
        assert_eq!(ltr.files.iter().nth(0).unwrap(), file_1);
        assert_eq!(ltr.files.iter().nth(1).unwrap(), file_2);
        assert_eq!(ltr.date.unwrap(), date);
        assert_eq!(ltr.theme.unwrap(), theme);
    }
    let wo_addresse_2 = Path::new(ABSOLUTE_PATH)
    .join("ltrs")
    .join("wo_addresse_2.ltr");
    if let Ok(decoded) = super::open_file(&wo_addresse_2, None)
    {
        let lltr = Ltr::parse(&wo_addresse_2);
        error!("Ошибка разбора файла {} \r\n{}", &wo_addresse_2.display(), lltr.as_ref().err().unwrap());
        assert_eq!(lltr.is_err(), true);
    }
    let wo_addresse = Path::new(ABSOLUTE_PATH)
    .join("ltrs")
    .join("wo_addresse.ltr");
    if let Ok(decoded) = super::open_file(&wo_addresse, None)
    {
        let lltr = Ltr::parse(&wo_addresse);
        error!("Ошибка разбора файла {} \r\n{}", &wo_addresse.display(), lltr.as_ref().err().unwrap());
        assert_eq!(lltr.is_err(), true);
    }
    let wo_files = Path::new(ABSOLUTE_PATH)
    .join("ltrs")
    .join("wo_files.ltr");
    if let Ok(decoded) = super::open_file(&wo_files, None)
    {
        let lltr = Ltr::parse(&wo_files);
        error!("Ошибка разбора файла {} \r\n{}", &wo_files.display(), lltr.as_ref().err().unwrap());
        assert_eq!(lltr.is_err(), true);
    }
    let wo_theme = Path::new(ABSOLUTE_PATH)
    .join("ltrs")
    .join("wo_theme.ltr");
    if let Ok(decoded) = super::open_file(&wo_theme, None)
    {
        let lltr = Ltr::parse(&wo_theme).unwrap();
       
        assert_eq!(lltr.theme, None);
    }
    let wo_date = Path::new(ABSOLUTE_PATH)
    .join("ltrs")
    .join("wo_date.ltr");
    if let Ok(decoded) = super::open_file(&wo_date, None)
    {
        let lltr = Ltr::parse(&wo_date).unwrap();
       
        assert_eq!(lltr.date, None);
    }
    let xz_error = Path::new(ABSOLUTE_PATH)
    .join("in_docs")
    .join("54334177")
    .join("envelope.ltr");
    if let Ok(decoded) = super::open_file(&xz_error, None)
    {
        let lltr = Ltr::parse(&xz_error).unwrap();
       
        assert_eq!(lltr.date, None);
    }
}

#[test]
fn strange_error_ltr()
{
    logger::StructLogger::new_default();
    let xz_error = Path::new(ABSOLUTE_PATH)
    .join("54334177")
    .join("envelope.ltr");
    if let Ok(decoded) = super::open_file(&xz_error, None)
    {
        let lltr = Ltr::parse(&xz_error).unwrap();
       
        assert_eq!(lltr.date, None);
    }
}
#[test]
fn test_all_ltrs()
{
    logger::StructLogger::new_default();
    let dirs = Path::new(ABSOLUTE_PATH);
    let dirs_count = 100;
    let mut processed = 0;
    for d in get_entries(&dirs).unwrap()
    {
        if let Some(subentry) = get_entries(&d.path())
        {
            for f in subentry
            {
                if let Some(ext) = f.path().extension()
                {
                    if ext == "ltr"
                    {
                        if let Ok(decoded) = super::open_file(&f.path(), None)
                        {
                            let ltr = Ltr::parse(&f.path());
                            if ltr.is_ok()
                            {
                                processed = processed + 1;
                            }
                            else {
                                error!("{}, {}", &f.path().display(), ltr.err().unwrap());
                            }
                        }    
                    }
                }
            }
        }
    }
    assert_eq!(dirs_count, processed);
}

#[test]
//Была включена футура encoding и поэитому все файлы преобразовывались 
//в тот энкодинг что в них указан, но мне не так надо, надо удалить нэемспейс до десериализации
//в общем будет видно потом
fn win_1251_file()
{
    logger::StructLogger::new_default();
    let win = Path::new(ABSOLUTE_PATH)
    .join("54146104")
    .join("document.xml");
    if let Ok(decoded) = super::open_file(&win, None)
    {
        let clear_xml = super::cleary_xml_namespaces(&decoded.1);
        //let clear_xml = clear_xml.replace("encoding=\"windows-1251\"", "encoding=\"utf-8\"");
        write_string_to_file(&clear_xml, "decoded_win1251.xml");
        
        let de: Result<Communication, DeError> = quick_xml::de::from_str(&clear_xml);
        info!("{}", de.unwrap().header.unwrap().get_source().get_organization());
    }
}

#[test]
//Была включена футура encoding и поэитому все файлы преобразовывались 
//в тот энкодинг что в них указан, но мне не так надо, надо удалить нэемспейс до десериализации
//в общем будет видно потом
fn xz_encoding()
{
    logger::StructLogger::new_default();
    let win = Path::new(ABSOLUTE_PATH)
    .join("53865418")
    .join("document.xml");
    let decode =  super::open_file(&win, None);
    if let Ok(decode) = decode
    {
        let clear_xml = super::cleary_xml_namespaces(&decode.1);
        //let clear_xml = clear_xml.replace("encoding=\"windows-1251\"", "encoding=\"utf-8\"");
        write_string_to_file(&clear_xml, "decoded_xz.xml");
        
        let de: Result<Communication, DeError> = quick_xml::de::from_str(&clear_xml);
        info!("{}", de.unwrap().header.unwrap().get_source().get_organization());
    }
    else
    {
        error!("{}", decode.err().unwrap());
    }
}

#[test]
fn xz_encoding_packet()
{
    logger::StructLogger::new_default();
    let win = Path::new(ABSOLUTE_PATH)
    .join("53865418");

    let mut packet = Packet::parse(&win);

    if packet.get_error().is_none()
    {
        serialize(&packet, "test_packet_2_7_1.json", None);
    }
    else
    {
        error!("{}", packet.get_error().unwrap());
    }
}


#[test]
//Была включена футура encoding и поэитому все файлы преобразовывались 
//в тот энкодинг что в них указан, но мне не так надо, надо удалить нэемспейс до десериализации
//в общем будет видно потом
fn test_parse_container_2_5()
{
    logger::StructLogger::new_default();
    let win = Path::new(ABSOLUTE_PATH)
    .join("54146104");
    let mut packet = Packet::parse(&win);
    if packet.get_error().is_none()
    {
        serialize(&packet, "test_packet_2_2.json", None);
    }
    else
    {
        error!("{}", packet.get_error().unwrap());
    }
}
#[test]
fn test_parse_container_2_7_1()
{
    logger::StructLogger::new_default();
    let win = Path::new(ABSOLUTE_PATH)
    .join("СФ_53596025_1_1");
    let packet = Packet::parse(&win);
    if packet.get_error().is_none()
    {
        serialize(&packet, "test_packet_2_7_1.json", None);
    }
    else
    {
        error!("{}", packet.get_error().unwrap());
    }
}





#[test]
///ошибка 
///   "error": "Ошибка в сериализаторе serde: /home/dev/medo_testdata/medo/0/27466149/container/passport.xml, missing field `description`"
fn test_packet_27466149()
{
    logger::StructLogger::new_default();
    let win = PathBuf::from("/hard/xar/medo_testdata/0/27466149");
    let mut packet = Packet::parse(&win);
    if packet.get_error().is_none()
    {
        serialize(&packet, "test_packet_2_7_1.json", None);
    }
    else
    {
        error!("{}", packet.get_error().unwrap());
    }
}
#[test]
///ошибка 
///задвоение в поле destination, наконец то решено!
fn test_packet_53294501()
{
    logger::StructLogger::new_default();
    let win = PathBuf::from("/hard/xar/medo_testdata/0/53294501");
    let packet = Packet::parse(&win);
    if packet.get_error().is_none()
    {
        serialize(&packet, "test_packet_2_7_1.json", None);
    }
    else
    {
        error!("{}", packet.get_error().unwrap());
    }
}
#[test]
fn test_acknowledgment()
{
    logger::StructLogger::new_default();
    let win = PathBuf::from("/hard/xar/medo_testdata/1/52691083");
    let mut packet = Packet::parse(&win);
    if packet.get_error().is_none()
    {
        serialize(&packet, "test_acknowledgment.json", None);
    }
    else
    {
        error!("{}", packet.get_error().unwrap());
    }
}


fn test_all_dirs()
{
    logger::StructLogger::new_default();
    let mut bench = Instant::now();
    let path = Path::new(ABSOLUTE_PATH)
    .join("in_docs");
    let dirs = get_files(&path);
    if let Some(dirs) = dirs
    {
        for d in &dirs
        {
            let full: PathBuf = [&path.to_str().unwrap(), d.as_str()].iter().collect();
            let in_dir = std::fs::read_dir(full).unwrap();
            for f in in_dir
            {
                let f = f.unwrap();
                if f.path().extension().is_some() && f.path().extension().unwrap() == "xml"
                {
                    //Возникнет ошибка если файл не в utf-8
                    if let Ok(xml) = std::fs::read_to_string(f.path())
                    {
                        let clear_xml = super::cleary_xml_namespaces(&xml);
                        let de: Result<Communication, DeError> = quick_xml::de::from_reader(clear_xml.as_bytes());
                        if de.is_err()
                        {
                            logger::debug!("{}, {}", f.path().display(), de.err().unwrap())
                        }
                        else
                        {
                            logger::info!("Обработан файл {}, {}", &f.path().display(), de.as_ref().unwrap().header.as_ref().unwrap().get_source().get_organization());
                        }
                    }
                    //ловим ошибку тут и парсим файл как windows-1251
                    else
                    {
                        if let Ok(decoded) = open_file(&f.path(), None)
                        {
                            let clear_xml = super::cleary_xml_namespaces(&decoded.1);
                            let de: Result<Communication, DeError> = quick_xml::de::from_reader(clear_xml.as_bytes());
                            if de.is_err()
                            {
                                logger::debug!("{}, {}", f.path().display(), de.err().unwrap())
                            }
                            else
                            {
                                logger::info!("Обработан файл {}, {}", &f.path().display(), de.as_ref().unwrap().header.as_ref().unwrap().get_source().get_organization());
                            }
                        }
                        else
                        {
                            logger::debug!("Ошибка декодирования файла, {}", f.path().display())
                        }
                    }
                }
            }
        }
        logger::debug!("За {:.2?} обработано {} директорий",bench.elapsed(), dirs.len())
    }
}


#[test]
fn test_all_dirs_as_packets()
{
    let errors_path = "/hard/xar/medo_testdata/errors";
    let c_in_data = Path::new("/hard/xar/medo_testdata/medo/0");
    logger::StructLogger::new_default();
    let bench = Instant::now();
    //let path = Path::new(ABSOLUTE_PATH)
    //.join("in_docs");
    let dirs = get_entries(&c_in_data);
    if let Some(dirs) = dirs
    {
        for d in &dirs
        {
            //let full: PathBuf = [&path.to_str().unwrap(), d.as_str()].iter().collect();
            let packet = Packet::parse(&d.path());
            if packet.get_error().is_none()
            {
                //write_json(&p, "test_packet_2_7_1.json");
                info!("Успешно обработан пакет {} версии {}", &d.path().display(), &packet.get_xml().as_ref().unwrap().communication.version)
            }
            else
            {
                let p = &packet;
                error!("{}", p.get_error().unwrap());
                let f_name = [p.get_packet_name(), ".json"].concat();
                serialize(p, &f_name, Some(errors_path))
            }
        }
        let secs = bench.elapsed().as_secs() as usize;
        let for_sec = dirs.len() / secs;
        logger::debug!("За {:.2?} обработано {} директорий, ({} директорий в секунду)",bench.elapsed(), dirs.len(), for_sec);
    }
}

#[test]
fn test_rc_files()
{
    logger::StructLogger::new_default();
    let errors_path = "/hard/xar/medo_testdata/errors";
    let d = PathBuf::from("/hard/xar/medo_testdata/0/П-У-738-22-ZZZ-268-04PUOEFQ2B");
    let  packet = Packet::parse(&d);
    if packet.get_error().is_none()
    {
        //write_json(&p, "test_packet_2_7_1.json");
        info!("Успешно обработан пакет {} версии {}", &d.display(), &packet.get_rc().as_ref().unwrap().content.as_ref().unwrap())
    }
    else
    {
        let p = &packet;
        error!("{}", p.get_error().unwrap());
        let f_name = [p.get_packet_name(), ".json"].concat();
        serialize(p, &f_name, Some(errors_path))
    }
}


#[test]
fn test_converting_rc()
{
    let errors_path = "/hard/xar/medo_testdata/errors";
    let converted_path = "//hard/xar/medo_testdata/converted";
    let c_in_data = PathBuf::from("/hard/xar/medo_testdata/0");
    logger::StructLogger::new_default();
    let d = PathBuf::from("/hard/xar/medo_testdata/0/П-У-738-22-ZZZ-268-04PUOEFQ2B");
    let packet = Packet::parse(&d);
    if packet.get_error().is_none()
    {
        //write_json(&p, "test_packet_2_7_1.json");
        info!("Успешно обработан пакет {} типа {}", &d.display(), &packet.get_rc().as_ref().unwrap().barcode.as_ref().unwrap());
        let conv : PacketInfo = packet.borrow().into();
        let f_name = [packet.get_packet_name(), ".json"].concat();
        serialize(conv, &f_name, Some(converted_path));
    }
    else
    {
        let p = &packet;
        error!("{}", p.get_error().unwrap());
        let f_name = [p.get_packet_name(), ".json"].concat();
        serialize(p, &f_name, Some(errors_path))
    }
}

#[test]
fn test_converting_2_7_1()
{
    let errors_path = "/hard/xar/medo_testdata/errors";
    let converted_path = "/hard/xar/medo_testdata/converted";
    let c_in_data = PathBuf::from("/hard/xar/medo_testdata/0");
    logger::StructLogger::new_default();
    let d = PathBuf::from("/hard/xar/projects/fullstack/complite_in_parser/test_data/in_docs/271/53580416_1");
    let packet = Packet::parse(&d);
    if packet.get_error().is_none()
    {
        //write_json(&p, "test_packet_2_7_1.json");
        info!("Успешно обработан пакет {} версии {}", &d.display(), &packet.get_xml().as_ref().unwrap().communication.version);
        let conv : PacketInfo = packet.borrow().into();
        let f_name = [packet.get_packet_name(), ".json"].concat();
        serialize(conv, &f_name, Some(converted_path));
    }
    else
    {
        let p = &packet;
        error!("{}", p.get_error().unwrap());
        let f_name = [p.get_packet_name(), ".json"].concat();
        serialize(p, &f_name, Some(errors_path))
    }
}

#[test]
fn test_converting_ack()
{
    let errors_path = "/hard/xar/medo_testdata/errors";
    let converted_path = "/hard/xar/medo_testdata/converted";
    let c_in_data = PathBuf::from("/hard/xar/medo_testdata/0");
    logger::StructLogger::new_default();
    let d = PathBuf::from("/hard/xar/medo_testdata/0/52690151");
    let packet = Packet::parse(&d);
    if packet.get_error().is_none()
    {
        //write_json(&p, "test_packet_2_7_1.json");
        info!("Успешно обработан пакет {} версии {}", &d.display(), &packet.get_xml().as_ref().unwrap().communication.version);
        let conv : PacketInfo = packet.borrow().into();
        let f_name = [packet.get_packet_name(), ".json"].concat();
        serialize(conv, &f_name, Some(converted_path));
    }
    else
    {
        let p = &packet;
        error!("{}", p.get_error().unwrap());
        let f_name = [p.get_packet_name(), ".json"].concat();
        serialize(p, &f_name, Some(errors_path))
    }
}


#[test]
fn test_wrong_xml()
{
    let errors_path = "/hard/xar/medo_testdata/errors";
    let converted_path = "/hard/xar/medo_testdata/converted";
    let c_in_data = PathBuf::from("/hard/xar/medo_testdata/0");
    logger::StructLogger::new_default();
    let d = PathBuf::from("/hard/xar/medo_testdata/0/29943971 - копия");
    let packet = Packet::parse(&d);
    if packet.get_error().is_none()
    {
        info!("Успешно обработан пакет {} версии {}", &d.display(), &packet.get_xml().as_ref().unwrap().communication.version);
        let conv : PacketInfo = packet.borrow().into();
        let f_name = [packet.get_packet_name(), ".json"].concat();
        serialize(conv, &f_name, Some(converted_path));
    }
    else
    {
        let p = &packet;
        error!("{}", p.get_error().unwrap());
        let f_name = [p.get_packet_name(), ".json"].concat();
        serialize(p, &f_name, Some(errors_path))
    }
}


#[test]
fn test_converting_2_5()
{
    let errors_path = "/hard/xar/medo_testdata/errors";
    let converted_path = "/hard/xar/medo_testdata/converted";
    let c_in_data = PathBuf::from("/hard/xar/medo_testdata/0");
    logger::StructLogger::new_default();
    let d = PathBuf::from("/hard/xar/medo_testdata/0/54139378");
    let mut packet = Packet::parse(&d);
    if packet.get_error().is_none()
    {
        //write_json(&p, "test_packet_2_7_1.json");
        info!("Успешно обработан пакет {} версии {}", &d.display(), &packet.get_xml().as_ref().unwrap().communication.version);
        let conv : PacketInfo = packet.borrow().into();
        let f_name = [packet.get_packet_name(), ".json"].concat();
        serialize(conv, &f_name, Some(converted_path));
    }
    else
    {
        let p = &packet;
        error!("{}", p.get_error().unwrap());
        let f_name = [p.get_packet_name(), ".json"].concat();
        serialize(p, &f_name, Some(errors_path))
    }
}


#[test]
fn test_convert_system_time()
{
    //logger::StructLogger::new_default();
    //let dt = std::time::SystemTime::now();
    //let converted = Date::from_system_time(dt);
    //logger::debug!("{}", converted);
}





#[test]
fn test_duplicate_field()
{
    let errors_path = "/hard/xar/medo_testdata/errors";
    let converted_path = "/hard/xar/medo_testdata/converted";
    //let c_in_data = PathBuf::from("/hard/xar/medo_testdata/0");
    logger::StructLogger::new_default();
    let d = PathBuf::from("/hard/xar/medo_testdata/error_duplicate_fields/2");
    let mut packet = Packet::parse(&d);
    if packet.get_error().is_none()
    {
        let attachments = packet.get_container().unwrap().attachments.as_ref().and_then(|a| Some(&a.attachments)).unwrap();
        for a in attachments
        {
            for s in a.signature.as_ref().unwrap()
            {
                info!("поле signature {}", &s.local_name);
            }
        } 
        //write_json(&p, "test_packet_2_7_1.json");
        info!("Успешно обработан пакет {} версии {}", &d.display(), &packet.get_xml().as_ref().unwrap().communication.version);
        let conv : PacketInfo = packet.borrow().into();
        let f_name = [packet.get_packet_name(), ".json"].concat();
        serialize(conv, &f_name, Some(converted_path));
    }
    else
    {
        let p = &packet;
        error!("{}", p.get_error().unwrap());
        let f_name = [p.get_packet_name(), ".json"].concat();
        serialize(p, &f_name, Some(errors_path))
    }
}








struct TextComplexEnum
{
    parser_name: String,
    parser_type: u32
}

enum TestEnum
{
    TestData(TextComplexEnum)
}
