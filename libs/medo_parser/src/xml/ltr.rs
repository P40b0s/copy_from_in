use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tokenizer::{ Lexer, GlobalActions, TokenActions, Tokenizer, ForwardTokenActions, TokenModel};
use tokenizer_derive::Tokenizer;
use crate::{MedoParserError, open_file, FileEncoding};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
///Структура для содержимого файла .ltr
pub struct Ltr
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_autosend: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_esd: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_delivered: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_readed: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotation_file: Option<String>,
    pub addresses: Vec<String>,
    pub files: Vec<String>,
}

impl Ltr
{
    pub fn parse(file: &PathBuf) -> Result<Ltr, crate::MedoParserError> 
    {
        let decoded = open_file(file, Some(FileEncoding::Windows1251))?;
        let ltr = parse_ltr(&decoded.1, file)?;
        Ok(ltr)
    }
}
impl Default for Ltr
{
    fn default() -> Self 
    {
        Ltr 
        {
            theme: None,
            is_autosend: None,
            is_esd: None,
            is_delivered: None,
            is_readed:None,
            date: None,
            annotation_file: None,
            addresses: vec![],
            files: vec![]
        }
    }
}


#[derive(Copy, PartialEq, Clone, Tokenizer)]
enum LtrTokens
{
    //[ПИСЬМО КП ПС СЗИ] ппц от уито пришло ПСИЬМО пришлось переделывать регекс
    #[token(pattern(r#"\[[ПИСЬМО]{6}.*\]"#))]
    Root,
    //ТЕМА=ЭСД МЭДО (78 от 25.01.2023 {9F50BC3D-47AC-4446-B528-678BB8FB0C30})
    #[token(pattern(r#"(?i)тема=([^\n\r]+)"#))]
    Theme,
    //АВТООТПРАВКА=1
    #[token(pattern(r#"(?i)автоотправка=([^\n\r]+)"#))]
    IsAutosend,
    //ЭЦП=0
    #[token(pattern(r#"(?i)эцп=([^\n\r]+)"#))]
    IsEds,
    //ДОСТАВЛЕНО=1
    #[token(pattern(r#"(?i)доставлено=([^\n\r]+)"#))]
    IsDelivered,
    //ПРОЧТЕНО=1
    #[token(pattern(r#"(?i)прочтено=([^\n\r]+)"#))]
    IsReading,
    //ДАТА=26.01.2023 13:22:52
    #[token(pattern(r#"(?i)дата=([^\n\r]+)"#))]
    Date,
    //[АДРЕСАТЫ]
    #[token(pattern(r#"(?i)\[АДРЕСАТЫ\]"#))]
    Addressees,
    //[ФАЙЛЫ]
    #[token(pattern(r#"\[ФАЙЛЫ\]"#))]
    Files,
    //хз зачем это обычно ссылка на аннотацию
    #[token(pattern(r#"\[ПИСЬМО.*\]"#))]
    File,
    //[ТЕКСТ]
    #[token(pattern(r#"\[ТЕКСТ\]"#))]
    Text,
    // (номер)ключ = значение
    #[token(pattern(r#"\d=([^\n\r]+)"#))]
    NumberKey
}

impl std::fmt::Display for LtrTokens 
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        match self
        {
            LtrTokens::Root => fmt.write_str("[ПИСЬМО КП ПС СЗИ]"),
            LtrTokens::Theme => fmt.write_str("ТЕМА"),
            LtrTokens::IsAutosend => fmt.write_str("АВТООТПРАВКА"),
            LtrTokens::IsEds => fmt.write_str("ЭЦП"),
            LtrTokens::IsDelivered => fmt.write_str("ДОСТАВЛЕНО"),
            LtrTokens::IsReading=> fmt.write_str("ПРОЧТЕНО"),
            LtrTokens::Date => fmt.write_str("ДАТА"),
            LtrTokens::Addressees => fmt.write_str("[АДРЕСАТЫ]"),
            LtrTokens::Files => fmt.write_str("[ФАЙЛЫ]"),
            LtrTokens::Text => fmt.write_str("[ТЕКСТ]"),
            LtrTokens::File => fmt.write_str("ФАЙЛ"),
            LtrTokens::NumberKey => fmt.write_str("0=значение"),
        }
    }
}
fn parse_ltr(data: &str, file: &PathBuf) -> Result<Ltr, MedoParserError>
{
    let mut ltr = Ltr::default();
    if let Some(defs) = LtrTokens::get_defs()
    {
        let actions = Lexer::tokenize(data, defs);
        ltr.date = get_field(LtrTokens::Date, &actions, file);
        ltr.theme = get_field(LtrTokens::Theme, &actions, file);

        if let Some(root) = actions.get(LtrTokens::Root)
        {
            let adr = add(&root, LtrTokens::Addressees, data, file)?;
            for a in adr
            {
                ltr.addresses.push(a);
            }
            
            let files = add(&root, LtrTokens::Files, data, file)?;
            for f in files
            {
                ltr.files.push(f);
            }
        }
        return Ok(ltr);
    }
    else
    {
        return Err(MedoParserError::ParseError("Ошибка компиляции регексов".to_owned()));
    }
    
}

fn add(start_token: &TokenModel<LtrTokens>, token: LtrTokens, data: &str, file: &PathBuf) -> Result<Vec<String>, MedoParserError>
{
    let header = start_token.find_forward(&[token], 15, false);
    if header.is_none()
    {
        //logger::error!("Не найдено свойство {} \r\n{}", token, data);
        return Err(MedoParserError::ParseError(format!("В файле {} не найдено свойство {} \r\n{}", file.display(), token, data)));
    }
    let header = header.unwrap();
    let list = header
    .take_forward_while(&[LtrTokens::NumberKey]);
    if list.len() == 0
    {
        //logger::error!("Не найдено ни одного значения {} \r\n{}", LtrTokens::NumberKey, data);
        return Err(MedoParserError::ParseError(format!("В файле {} не найдено ни одного значения {} \r\n{}", file.display(), LtrTokens::NumberKey, data)));
    }
    let mut values :Vec<String> = vec![];
    for val in &list
    {
        let gr = val.get_first_group();
        if gr.is_none()
        {
            //logger::error!("Не распознана группа токена addresses {} g:{0} \r\n{} ",val.token.value, data);
            return Err(MedoParserError::ParseError(format!("В файле {} не распознана группа токена addresses {} g:{0} \r\n{} ", file.display(), val.token.value, data)));
        }
        else
        {
            values.push(gr.unwrap().to_owned());
        }
    }
    Ok(values)

}

fn get_field(token: LtrTokens, actions: &GlobalActions<LtrTokens>, file: &PathBuf) -> Option<String>
{
    if let Some(date) = actions.get(token)
    {
        let gr = date.get_first_group();
        if gr.is_some()
        {
            return Some(gr.unwrap().to_owned());
        }
        else
        {
            logger::warn!("В файле {} отсутсвует необязательное поле {}", file.display(), token);
            return None;
        }
    }
    None
}

