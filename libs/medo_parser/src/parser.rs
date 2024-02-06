use std::path::PathBuf;

pub trait MedoParser where Self: Sized
{
    ///Расширение файла которое обрабатывается данным парсером
    const EXTENSION : &'static str;
    ///Файл который надо распарсить, и список вайлов которые могут быть найдены далее
    /// например в пакете 2.7 надо дополнительно парсить файлы из архива, 
    /// вот их и надо добавить с этот список
    fn parse(file: &PathBuf, paths: Option<&mut Vec<PathBuf>>) -> Result<Self, super::MedoParserError>;
}