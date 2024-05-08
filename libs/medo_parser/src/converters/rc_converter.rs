use utilites::Date;
use crate::{medo_model::{PacketInfo, Requisites, SenderInfo}, DatesHelper, RcParser};
use super::converter::UniversalConverter;


impl UniversalConverter for RcParser
{
    fn convert(&self, to: &mut PacketInfo) 
    {
        to.packet_type = Some("rc".to_owned());
        let mut req = Requisites::default();
        req.document_guid = self.guid.clone();
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
        let s = SenderInfo 
        {
            medo_addressee : Some("неизвестно".to_owned()),
            organization : Some("неизвестно".to_owned()),
            source_guid : Some("00000000-0000-0000-0000-000000000000".to_owned()),
            ..SenderInfo::default()
        };
        to.sender_info = Some(s);
        logger::error!("В пакете rc отсутсвуют свойства отправителя {}", &to.packet_directory);
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

