use utilites::Date;
use crate::{medo_model::{PacketInfo, Requisites, SenderInfo}, RcParser};
use super::converter::UniversalConverter;


impl UniversalConverter for RcParser
{
    fn convert(&self, to: &mut PacketInfo) 
    {
       
        to.packet_type = Some("rc".to_owned());
        let mut req = Requisites::default();
        //в rc приходят не те форты guid что я хнаю в БД поэтому парсим их и переводим в нормальную.
        if let Some(duid) = &self.guid
        {
            let document_guid = uuid::Uuid::parse_str(duid);
            req.document_guid = document_guid.ok().and_then(|g| Some(g.to_string()));
        }
        req.act_type = self.viddoc.clone();
        req.pages = self.pages_orig;
        req.document_number = self.regnumber.clone();
        //15.10.2022
        if let Some(date) = self.regdate.as_ref()
        {
            if let Some(date) = Date::parse(date)
            {
                req.sign_date = Some(date.format(utilites::DateFormat::Serialize));
            }
        }
        req.annotation =  self.content_2.clone();
        to.requisites = Some(req);
        if let Some(org) = self.signer_org.as_ref()
        {
            let (addr, uid) = if org == "Президент РФ"
            {
                logger::warn!("В пакете rc {} установлены свойства администрации президента", &to.packet_directory);
                (Some("ADM_PREZ~MEDOGU".to_owned()), Some("0b21bba1-f44d-4216-b465-147665360c06".to_owned()))
            }
            else
            {
                logger::warn!("В пакете rc {} не найдены свойтсва отправителя", &to.packet_directory);
                (None, None)
            };
            to.sender_info = Some(SenderInfo 
            {
                medo_addressee : addr,
                organization : Some(org.to_owned()),
                source_guid : uid,
                ..SenderInfo::default()
            });
        }
        if to.default_pdf.is_none()
        {
            let mut pdfs = to.files.iter().filter(|f| f.contains(".pdf"));
            if let Some(pdf) = pdfs.next()
            {
                to.default_pdf = Some(pdf.clone());
            }
            else 
            {
                logger::warn!("В транспортном пакете отсутствует файл pdf");
            }
        }

    }
}

