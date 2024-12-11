use crate::{medo_model::PacketInfo, Packet};
use utilites::{Date, DateFormat};

pub trait UniversalConverter
{
    fn convert(&self, to: &mut PacketInfo);
}

impl From<&Packet> for PacketInfo
{
    fn from(value: &Packet) -> Self
    {
        
        let mut info = PacketInfo::default();
        if let Some(files) = value.get_packet_files()
        {
            info.files = files.to_owned();
        }
        info.packet_directory = value.get_packet_name().to_owned();
        if let Some(dt) = value.get_packet_date_time()
        {
            info.delivery_time = dt.into_owned();
        }
        if let Some(xml) = value.get_xml()
        {
            xml.convert(&mut info);
        }
        if let Some(rc) = value.get_rc()
        {
            rc.convert(&mut info);
        }
        info.error = value.get_error().as_ref().and_then(|e| Some(e.to_string()));
        info.update_key = Date::now().format(DateFormat::Serialize);
        //FIXME исправлено, тут мы не можем зать полный путь к пакету
        // if info.default_pdf.is_some()
        // {
        //     let path = Path::new(&info.packet_directory).join(info.default_pdf.as_ref().unwrap());
        //     info.pdf_hash = utilites::Hasher::hash_from_path(path);
        // }
        info
    }
}

impl From<Packet> for PacketInfo
{
    fn from(value: Packet) -> Self 
    {
        let mut info = PacketInfo::default();
        if let Some(files) = value.get_packet_files()
        {
            info.files = files.to_owned();
        }
        info.packet_directory = value.get_packet_name().to_owned();
        if let Some(dt) = value.get_packet_date_time()
        {
            info.delivery_time = dt.into_owned();
        }
        if let Some(xml) = value.get_xml()
        {
            xml.convert(&mut info);
        }
        if let Some(rc) = value.get_rc()
        {
            rc.convert(&mut info);
        }
        info.error = value.get_error().as_ref().and_then(|e| Some(e.to_string()));
        info.update_key = Date::now().format(DateFormat::Serialize);
        
        // if info.default_pdf.is_some()
        // {
        //     let path = Path::new(&info.packet_directory).join(info.default_pdf.as_ref().unwrap());
        //     info.pdf_hash = utilites::Hasher::hash_from_path(path);
        // }
        info
    }
}


// fn check_trace(info: &mut PacketInfo)
// {
//     let tr = TRACES.lock().unwrap();
//     let trace_list = tr.get_trace_list_as_ref();
//     let mut finded : Option<TracePacket> = None;
//     if !trace_list.is_empty()
//     {
//         for t in trace_list
//         {
//             if let Some(s) = info.sender_info.as_ref()
//             {
//                 if &t.sender_id == s.source_guid.as_ref().unwrap_or(&String::new())
//                 {
//                     if let Some(reg) = info.requisites.as_ref()
//                     {
//                         if &t.doc_number == reg.document_number.as_ref().unwrap_or(&String::new())
//                         {
//                             info.trace_message = Some(t.comment.clone());
//                             finded = Some(t.clone());
//                             debug!("Обнаружен документ из списка отслеживания -> header_id:{}", info.header_guid.as_ref().unwrap_or(&String::new()));
//                             break;
//                         }
//                         if let Some(mj) = reg.mj.as_ref()
//                         {
//                             if mj.number == t.doc_number
//                             {
//                                 info.trace_message = Some(t.comment.clone());
//                                 finded = Some(t.clone());
//                                 debug!("Обнаружен документ из списка отслеживания -> header_id:{}", info.header_guid.as_ref().unwrap_or(&String::new()));
//                                 break;
//                             }
//                         }
//                     }
//                 }
//             }  
//         }
//         if let Some(finded) = finded
//         {
//             drop(tr);
//             TRACES.lock()
//             .as_mut()
//             .unwrap()
//             .remove_trace_obj(&finded);
//         }
//     }
//}