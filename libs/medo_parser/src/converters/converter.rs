use crate::{medo_model::PacketInfo, Packet};
use settings::DateTimeFormat;

pub trait UniversalConverter
{
    fn convert(&self, to: &mut PacketInfo);
}

// pub trait MainConverter
// {
//     ///Конвертирование исходных данных в данные необходимые нам
//     ///Является многопоточным
//     fn convert_packets(packet: &Vec<Packet>, current_dirs: Dirs, processing : Option<fn(&mut PacketInfo, dirs: Dirs)>, external_sender: Sender<ParserStatus>) -> Vec<PacketInfo>
//     {
//         let result = threaded_converted_packets(packet, current_dirs, processing, external_sender);
//         result
//     }
// }

//Многопоточный парсинг директории в которой есть транспортрные пакеты
//Убрал пока этот функционал
// fn threaded_converted_packets(packets: &Vec<Packet>, current_dirs: Dirs, processing : Option<fn(&mut PacketInfo, dirs: Dirs)>, external_sender: Sender<ParserStatus>) -> Vec<PacketInfo>
// {
//     let len = packets.len();
//     let mut results: Vec<PacketInfo> = vec![]; 
//     if len == 0
//     {
//         return results;
//     }
//     CURRENT_PARSER_STATUS.write().unwrap().new_convert_run();
//     let _ = external_sender.send(CURRENT_PARSER_STATUS.read().unwrap().clone());
//     let bench = Instant::now();
//     let len = packets.len();
//     let mut  chunk_size = 1usize;
//     if len > 100
//     {
//         chunk_size = 2;
//     }
//     if len > 1000
//     {
//         chunk_size = 10;
//     }
//     //Делим массив на указанное количество частей, если ровно поделить не удается
//     //то последний массив будет содержать итемы что остались от остатка от деления
//     let split  = packets.len() / chunk_size;
//     let arrays = packets.chunks(split);
//     let mut handles : Vec<JoinHandle<()>> = vec![];
//     let (sender, receiver) = channel();
//     let mut running_thread_count = 1; 
//     for arr in arrays
//     {
//         let dirs = current_dirs.clone();
//         let s = sender.clone();
//         let pac : Vec<Packet> = arr.iter().map(|m|m.to_owned()).collect();
//         handles.push(std::thread::spawn(move ||
//         {
//             for p in pac
//             {
//                 let dirs = dirs.clone();
//                 let mut pi: PacketInfo = p.borrow().into();
//                 if let Some(prc) = processing
//                 {
//                     prc(&mut pi, dirs)
//                 }
//                 let _s = s.send(pi);
//             }
//         }));
//         running_thread_count = running_thread_count + 1;
//     }
//     drop(sender);
//     let mut index = 1.0f32;
//     let len = packets.len() as f32;
//     for r in receiver
//     {
//         let external_sender_clone = external_sender.clone();
//         let perc:f32 = (index*100.0)/len;
//         logger::debug!("Конвертирование {}%", perc.round());
//         CURRENT_PARSER_STATUS.write().unwrap().set_convert_percentage(perc.round() as u32);
//         if perc.round() as u32 % 5 == 0
//         {
//             let _ = external_sender_clone.send(CURRENT_PARSER_STATUS.read().unwrap().clone());
//         }
//         index = index + 1.0;
//         results.push(r);
//     }
//     let mut i = 0;
//     CURRENT_PARSER_STATUS.write().unwrap().set_convert_percentage(100);
//     let _ = external_sender.send(CURRENT_PARSER_STATUS.read().unwrap().clone());
//     drop(external_sender);
//     for h in handles
//     {
//         let j = h.join();//.expect("Не получается присоединить один из потоков");
//         if j.is_err()
//         {
//             logger::error!("Ошибка объединения потока {i}");
//         }
//         i = i + 1;
//     }
//     let len =  packets.len();
//     let secs = bench.elapsed().as_secs() as usize;
//     let for_sec = match secs
//     {
//         0 => len,
//         _ => len / secs
//     };
//     logger::debug!("За {:.2?} переконвертировано {} пакетов, ({} пакетов за секунду)",bench.elapsed(), len, for_sec);
//     results
// }


//impl MainConverter for PacketInfo{}
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
        let err = value.get_error();
        if let Some(err) = err
        {
            info.error = Some(err.into_owned());
        }
        info.update_key = settings::Date::now().write(settings::DateFormat::Serialize);
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