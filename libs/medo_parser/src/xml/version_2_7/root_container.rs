use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename="container")]
#[serde(rename_all = "camelCase")]
///Буду использовать как вложение для контейнера
/// в рутовом xml тут расположен только путь к архиву с документов внутри
/// в архиве у него есть файл pasport.xml
/// этот файл надо распарсить и то что получилось добавить к этой модели
/// будет не так как в основной структуре, но нам это и не важно
pub struct RootContainer
{
    pub body: String,
}