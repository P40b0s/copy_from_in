use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use crate::{Uid, guid_deserializer};

// <c:acknowledgment c:uid="8084c1c4-db4d-4a0a-b0f0-2711ab0c073c">
// <c:time>2023-01-10T20:57:02+03:00</c:time>
// <c:accepted>true</c:accepted>
// </c:acknowledgment>

// <xdms:acknowledgment xdms:uid="00000000-0000-0000-0000-000000000000" xdms:content="MEDOErrorAcknowledgment">
// <xdms:time>2023-01-03T13:34:06+03:00</xdms:time>
// <xdms:accepted>false</xdms:accepted>
// <xdms:errorCode>248</xdms:errorCode>
// <xdms:errorFileMessage>originalMessage.zip</xdms:errorFileMessage>
// <xdms:comment>Для разработчика Вашего СЭД:Отсутствует идентификатор сообщения communication:header:@uid.</xdms:comment>
// </xdms:acknowledgment>

// <xdms:acknowledgment xdms:uid="4A45CD60-5AD4-487C-ACB9-85C5BE055B43" xdms:content="Квитанция на сообщение типа "Уведомление". Исх. №: 4312-р, Дата: 2022-12-29">
// <xdms:time>2023-01-03T18:05:04.917</xdms:time>
// <xdms:accepted>true</xdms:accepted>
// <xdms:comment>Сообщение доставлено</xdms:comment>
// </xdms:acknowledgment>

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Acknowledgment
{
    #[serde(deserialize_with="guid_deserializer")]
    #[serde(rename="@uid")]
    uid: String,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Например MEDOErrorAcknowledgment
    #[serde(rename="@content")]
    pub content: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///2023-01-03T18:05:04.917 <br>
    ///2023-01-03T13:34:06+03:00 <br>
    ///2023-01-10T20:57:02+03:00 <br>
    pub time: Option<String>,
    ///Аттирибут
    pub accepted: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code : Option<u32>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Приложение, архив с сообщением в котором обнаружена ошибка
    pub error_file_message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Коментарий
    pub comment: Option<String>
}

impl Acknowledgment
{
    pub fn is_error(&self) -> bool
    {
        self.error_file_message.is_some()
    }
}

impl Uid for Acknowledgment
{
    fn get_uid(&self) -> Cow<str>
    {
        Cow::from(&self.uid)
    }
}