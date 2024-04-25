use crate::{medo_model::{Ack, Executor, MinistryOfJustice, PacketInfo, Requisites, SenderInfo}, DatesHelper, Uid, XmlParser};
use logger::warn;
use utilites::Date;
use super::UniversalConverter;

impl UniversalConverter for XmlParser
{
    fn convert(&self, to: &mut PacketInfo) 
    {
        if !identification(self, to)
        {
            return;
        }
        if self.is_acknowledgment()
        {
            convert_acknowledgment(self,  to);
            return;
        }
        if self.is_container()
        {
            convert_container(self, to);
        }
        else
        {
            convert_root(self,  to)
        }
    }
}

///Проверяем, если это стандатная схема мэдо, то возвращаем true, если нет то false
fn identification(value: &XmlParser, packet: &mut PacketInfo) -> bool
{
    //io::serialize(value, "TEST_ACK", None);
    let mut sender = SenderInfo::default();
    packet.wrong_encoding = value.wrong_encoding; 
    if let Some(header) = value.get_header()
    {
        let source = header.get_source();
        let organization = source.get_organization();
        let uid = header.get_uid();
        if uid == "00000000-0000-0000-0000-000000000000"
        {
            packet.header_guid = Some(uuid::Uuid::new_v4().to_string());
        }
        else
        {
            packet.header_guid = Some(uid.into_owned());
        }
        packet.packet_type = header.get_type().into_owned();
        //организация отправившая пакет и ее uid
        sender.organization = Some(organization.into_owned());
        sender.source_guid = Some(source.get_uid().into_owned());
        sender.medo_addessee = match value.get_medo_addressee()
        {
            Some(e) => Some(e.into_owned()),
            None => 
            {
                warn!("Не найден адрес мэдо для source id {}", sender.source_guid.as_ref().unwrap());
                None
            }
        };
        packet.sender_info = Some(sender);
        return true;
    }
    return false;
}

fn convert_acknowledgment(xml: &XmlParser, info: &mut PacketInfo)
{
    if let Some(ack) = xml.get_acknowledgment()
    {
        let a = Ack
        {
            comment: ack.comment.clone(),
            accepted: ack.accepted,
            time: ack.time.clone()
        };
        info.acknowledgment = Some(a);
    }
}
fn convert_root(xml: &XmlParser, info: &mut PacketInfo)
{
    if let Some(doc) = xml.get_document()
    {
        let mut req = Requisites::default();
        req.document_guid = Some(doc.get_uid().into_owned());
        req.act_type = doc.kind.clone();
        req.pages = doc.pages;
        if let Some(annotation) = &doc.annotation
        {
            if let Some(mj_req) = Date::extract_mj_requisites(annotation)
            {
                let mj = MinistryOfJustice
                {
                    number : mj_req.0,
                    date : mj_req.1
                };
                req.mj = Some(mj);
            }
        }
        req.annotation = doc.annotation.clone();
        if let Some(num) = doc.num.as_ref()
        {
            req.sign_date = num.date.clone();
        }
        if let Some(num) = doc.num.as_ref()
        {
            req.document_number = num.number.clone();
        }
        info.requisites = Some(req);
        if let Some(signs) = doc.signatories.as_ref()
        {
            if let Some(s) = signs.signatories.first()
            {
                if let Some(sender) = info.sender_info.as_mut()
                {
                    sender.department = s.department.clone();
                    sender.person = s.person.clone();
                    sender.post = s.post.clone();
                }
            }
        }
        if let Some(exe) = doc.executor.as_ref()
        {
            if let Some(sender) = info.sender_info.as_mut()
            {
                let executor = Executor
                {
                    organization: exe.organization.clone(),
                    person: exe.person.clone(),
                    post: exe.post.clone(),
                    contact_info: exe.contact_info.clone()
                };
                sender.executor = Some(executor);
            }
        }
        if let Some(files) = xml.communication.files.as_ref()
        {
            for f in &files.files
            {
                if f.local_name.contains(".pdf")
                {
                    if info.files.contains(&f.local_name)
                    {
                        info.default_pdf = Some(f.local_name.clone());
                    }
                    else 
                    {
                        logger::warn!("Файл pdf {} указанный в сопроводительном файле отсутсвует в транспортном пакете", &f.local_name);
                    }
                }
            }
        }
        if info.default_pdf.is_none()
        {
            let mut pdfs = info.files.iter().filter(|f| f.contains(".pdf"));
            if let Some(pdf) = pdfs.next()
            {
                info.default_pdf = Some(pdf.clone());
            }
            else 
            {
                logger::warn!("В транспортном пакете отсутствует файл pdf");
            }
        }
        //Расчитываем хэш pdf файла
        //super::calculate_default_pdf_hash(info);
        // if info.default_pdf.is_some()
        // {
        //     info.pdf_hash = calculate_pdf_hash(&info.packet_directory, info.default_pdf.as_ref().unwrap());
        // }
    }
}

fn convert_container(xml: &XmlParser, info: &mut PacketInfo)
{
    if let Some(cnt) = xml.get_container()
    {
        let mut req = Requisites::default();
        req.document_guid = Some(cnt.uid.clone());
        req.act_type = Some(cnt.requisites.document_kind.clone());
        req.pages = Some(cnt.document.pages_quantity);
        req.annotation = Some(cnt.requisites.annotation.clone());
        if let Some(auhtor) = cnt.authors.authors.first()
        {
            if let Some(sender) = info.sender_info.as_mut()
            {
                sender.organization = Some(auhtor.organization.title.clone());
                sender.department = auhtor.department.clone();
                //TODO берется только первый подписант, может остальные и не нужны
                sender.person = Some(auhtor.sign.first().unwrap().person.name.clone());
                sender.post = Some(auhtor.sign.first().unwrap().person.post.clone());
                sender.addressee = auhtor.organization.address.clone();
                if let Some(exe) = auhtor.executor.as_ref()
                {
                    let executor = Executor
                    {
                        person: exe.name.clone(),
                        post: exe.post.clone(),
                        contact_info: 
                        {
                            let phone = exe.phone.as_ref().map_or("", |m| m.as_str());
                            let mail = exe.email.as_ref().map_or("", |m| m.as_str());
                            Some([phone, " ", mail].concat())
                        },
                        organization: None
                    };
                    sender.executor = Some(executor);
                }
                req.document_number = auhtor.registration.number.clone();
                req.sign_date = auhtor.registration.date.clone();
            }
        }
        info.requisites = Some(req);
        let local_file = &cnt.document.local_name;

        //Ищем preview нам нужен он
        let mut pdfs = info.files.iter().filter(|f| f.contains("preview"));
        if let Some(pdf) = pdfs.next()
        {
            info.default_pdf = Some(pdf.clone());
        }
        //не находим preview то делаем дефолтным файл который указан в сопроводиловке
        if info.default_pdf.is_none()
        {
            if local_file.contains(".pdf")
            {
                let mut files = info.files.iter().filter(|f|f.contains(local_file));
                if let Some(f) = files.next()
                {
                    info.default_pdf = Some(f.to_owned());
                }
                else 
                {
                    logger::warn!("Файл pdf {} указанный в сопроводительном файле отсутсвует в транспортном пакете", local_file);
                }
            }
        }
        //делаем дефолтным первый попавшийся pdf файл
        if info.default_pdf.is_none()
        {
            let mut pdfs = info.files.iter().filter(|f| f.contains(".pdf"));
            if let Some(pdf) = pdfs.next()
            {
                info.default_pdf = Some(pdf.clone());
            }
            else 
            {
                logger::warn!("В транспортном пакете отсутствует файл pdf");
            }
        }
        //Расчитываем хэш pdf файла
        //super::calculate_default_pdf_hash(info);
    }
}

