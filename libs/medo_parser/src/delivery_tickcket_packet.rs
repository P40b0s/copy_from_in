use std::{fs, path::Path};
use encoding::{all::WINDOWS_1251, Encoding};
use utilites::DateFormat;

pub struct DeliveryTicketPacket
{
    header_uid: String,
    xml_file: Vec<u8>,
    envelope_file: Vec<u8>
}
impl DeliveryTicketPacket
{
    /// __ack_uid__ - UID документа который к нам пришел  
    /// __destination_organ_uid__ - UID органа в который будет отправлятся уведомление  
    /// __destination_organ_name__ - наименование органа в который будет отправлятся уведомление  
    /// __medo_addr__ - адрс МЭДО
    pub fn create_packet(ack_uid: &str, destination_organ_uid :&str, destination_organ_name: &str, medo_addr: &str) -> Self
    {
        let xml = create_ticket(ack_uid, destination_organ_uid, destination_organ_name);
        let envelope = create_enveloper(medo_addr);
        Self { header_uid: ack_uid.to_owned(), xml_file: xml.into_bytes(), envelope_file: envelope }
    }
    pub fn send<S: AsRef<Path>>(&self, path: S)
    {
        let dir_name = ["Квитанция_uid_", &self.header_uid].concat();
        let dir = Path::new(path.as_ref()).join(&dir_name);
        let _ = fs::create_dir(&dir);
        if dir.exists()
        {
            let xml_path = dir.as_path().join("acknowledgment.xml");
            let env_path = dir.as_path().join("envelope.ltr");
            let _ = fs::write(&xml_path, &self.xml_file);
            let _ = fs::write(&env_path, &self.envelope_file);
        }
    }
}



fn create_ticket(ack_uid: &str, destination_organ_uid :&str, destination_organ_name: &str) -> String
{
    //let date_time = SystemTime::now();
    let created_datetime = utilites::Date::now().format(DateFormat::Serialize);
    let header_uid = uuid::Uuid::new_v4().to_string();

    let ticket = format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>
<c:communication c:version=\"2.7.1\" xmlns:c=\"urn:IEDMS:MESSAGE\">
    <c:header c:type=\"Квитанция\" c:uid=\"{}\" c:created=\"{}+03:00\">
        <c:source c:uid=\"1953a86e-e35e-45cc-b031-556ca72c4080\">
            <c:organization>Управление обеспечения правовой информатизации Службы специальной связи Федеральной службы охраны Российской Федерации (УОПИ Спецсвязи ФСО России)</c:organization>
        </c:source>
    </c:header>
    <c:acknowledgment c:uid=\"{}\">
        <c:time>{}+03:00</c:time>
        <c:accepted>true</c:accepted>
    </c:acknowledgment>
    <c:deliveryIndex>
        <c:destination>
            <c:destination c:uid=\"{}\">
                <c:organization>{}</c:organization>
            </c:destination>
        </c:destination>
    </c:deliveryIndex>
</c:communication>", header_uid, &created_datetime, ack_uid, &created_datetime, destination_organ_uid, destination_organ_name);
    return ticket;
}
fn create_enveloper(addresse: &str) -> Vec<u8>
{
    let ticket = format!("[ПИСЬМО КП ПС СЗИ]
ТЕМА=Квитанция о приеме
ШИФРОВАНИЕ=0
ЭЦП=1
ДОСТАВЛЕНО=1
ПРОЧТЕНО=1
[АДРЕСАТЫ]
0={}
[ФАЙЛЫ]
0=acknowledgment.xml", addresse);
    let encoded = WINDOWS_1251.encode(&ticket, encoding::EncoderTrap::Replace);
    return encoded.unwrap()
}
#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    pub fn test_xml()
    {
        DeliveryTicketPacket::create_packet("ид дока который пришел", "ид органа которому направляем", "наименование органа которму направляем", "ooo@lll.ru").send("/hard/xar/projects/fullstack/complite_in_parser/medo_parser");
    }
}