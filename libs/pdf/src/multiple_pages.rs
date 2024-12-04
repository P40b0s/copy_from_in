// use logger::{error, backtrace, debug};
// use pdfium_render::prelude::{PdfPage, PdfPageRenderRotation, PdfRenderConfig, Pdfium};
// use tokio::sync::{futures, Mutex};
// use once_cell::sync::Lazy;

// use std::{io::Cursor, path::{Path, PathBuf}, sync::{mpsc::channel, Arc, RwLock}};
// use image::ImageFormat;

// use crate::{error, status::PdfProcessingStatus};

// static CONFIG_THUMBS: Lazy<Arc<PdfRenderConfig>> = Lazy::new(||Arc::new(PdfRenderConfig::new().set_target_width(256).set_maximum_height(256).rotate_if_landscape(PdfPageRenderRotation::Degrees90, true)));
// static CONFIG_ALL: Lazy<Arc<PdfRenderConfig>> = Lazy::new(||Arc::new(PdfRenderConfig::new().set_target_width(800).set_maximum_height(800).rotate_if_landscape(PdfPageRenderRotation::Degrees90, true)));
// pub static PDF_SERVICE_STATUS: Lazy<std::sync::Mutex<PdfProcessingStatus>> = Lazy::new(||std::sync::Mutex::new(PdfProcessingStatus::default()));


// fn get_instance() -> Result<Pdfium, error::Error> 
// {
//     let lib_paths = vec!
//     [
//         //путь для debug
//         Pdfium::pdfium_platform_library_name_at_path("libs/pdf/libs/"),
//         Pdfium::pdfium_platform_library_name_at_path("pdf/libs/"),
//         //обычный путь
//         Pdfium::pdfium_platform_library_name_at_path("./libs/")
//     ];
//     let mut last_err : pdfium_render::prelude::PdfiumError = pdfium_render::prelude::PdfiumError::UnrecognizedPath;
//     for path in lib_paths
//     {
//         let binding_result = Pdfium::bind_to_library(path);
//         if let Ok(result) = binding_result
//         {
//             return Ok(Pdfium::new(result));
//         }
//         else 
//         {
//             last_err = binding_result.err().unwrap();   
//         }
//     }
//     error!("{}", last_err.to_string());
//     return Err(error::Error::PdfiumError(last_err)); 
// }

// fn process_internal(path: &str, config: &PdfRenderConfig, page: Option<u32>) -> Result<Vec<String>, error::Error> 
// {
//     let pdfium = get_instance()?;
//     let document = pdfium.load_pdf_from_file(path, None)?;
//     let mut images : Vec<String> = vec![];
//     let pages_count = document.pages().len();
//     PDF_SERVICE_STATUS.lock().unwrap().set_pages(pages_count as u32);
//     if page.is_some()
//     {
//         let page = page.unwrap();
//         if page < 1 || page > pages_count as u32
//         {
//             let error = format!("Ошибка, страница для документа {} не может быть меньше 1 и больше {} (выбрана {})", path, pages_count, page);
//             PDF_SERVICE_STATUS.lock().unwrap().add_message(&error);
//             return Err(error::Error::WrongPageSelect(path.to_owned(), pages_count as u32, page));
//         }
//         let searched_page = (page -1) as usize;
//         let page = document.pages().iter().nth(searched_page).unwrap();
//         let rendered_page = render_page(&page, path, searched_page, &config)?;
//         let base_png = utilites::Hasher::from_bytes_to_base64(&rendered_page);
//         images.push(base_png);
//         PDF_SERVICE_STATUS.lock().unwrap().set_percentage(1 as u32, 1 as u32);
//     }
//     else 
//     {
//         for (index, page) in document.pages().iter().enumerate()
//         {
//             let rendered_page = render_page(&page, path, index, &config)?;
//             let base_png = utilites::Hasher::from_bytes_to_base64(&rendered_page);
//             images.push(base_png);
//             PDF_SERVICE_STATUS.lock().unwrap().set_percentage((index + 1) as u32, pages_count as u32);
//         }
//     }
//     Ok(images)
// }

// fn process_internal_to_u8(path: &str, config: &PdfRenderConfig, page: Option<u32>) -> Result<Vec<Vec<u8>>, error::Error> 
// {
//     let pdfium = get_instance()?;
//     let document = pdfium.load_pdf_from_file(path, None)?;
//     let mut images : Vec<Vec<u8>> = vec![vec![]];
//     let pages_count = document.pages().len();
//     PDF_SERVICE_STATUS.lock().unwrap().set_pages(pages_count as u32);
//     if page.is_some()
//     {
//         let page = page.unwrap();
//         if page < 1 || page > pages_count as u32
//         {
//             let error = format!("Ошибка, страница для документа {} не может быть меньше 1 и больше {} (выбрана {})", path, pages_count, page);
//             PDF_SERVICE_STATUS.lock().unwrap().add_message(&error);
//             return Err(error::Error::WrongPageSelect(path.to_owned(), pages_count as u32, page));
//         }
//         let searched_page = (page -1) as usize;
//         let page = document.pages().iter().nth(searched_page).unwrap();
//         let rendered_page = render_page(&page, path, searched_page, &config)?;
//         images.push(rendered_page);
//         PDF_SERVICE_STATUS.lock().unwrap().set_percentage(1 as u32, 1 as u32);
//     }
//     else 
//     {
//         for (index, page) in document.pages().iter().enumerate()
//         {
//             let rendered_page = render_page(&page, path, index, &config)?;
//             images.push(rendered_page);
//             PDF_SERVICE_STATUS.lock().unwrap().set_percentage((index + 1) as u32, pages_count as u32);
//         }
//     }
//     Ok(images)
// }

// // Извлечение страницы из pdf и преобразование ее в формат rgba8 pdf и выдача страницы в виде массива байт
// fn render_page<'a>(page: &'a PdfPage, path: &str, current_index: usize, config: &PdfRenderConfig) -> Result<Vec<u8>, error::Error>
// {
//     let current_page =  page.render_with_config(config)?;
//     let mut writer = std::io::BufWriter::new(Cursor::new(vec![]));
//     let dyn_image = current_page.as_image();
//     let rgba8 = dyn_image.as_rgba8();
//     if let Some(rgba) = rgba8
//     {
//         let _ = rgba.write_to(&mut writer, ImageFormat::Png)?;
//         let flash = writer.into_inner();
//         if let Ok(f) = flash
//         {
//             let buff = f.into_inner();
//             Ok(buff)
//         }
//         else 
//         {
//             return Err(error::Error::WriteBufferError(path.to_owned(), current_index as u32 + 1));
//         }
//     }
//     else 
//     {
//         return Err(error::Error::Rgba8ConvertError(path.to_owned(), current_index as u32 + 1));
//     }
// }

// //Тут делаем обработку ошибок из основной функции обработки и отдаем либо пустой массив либо с изображениями, ошибки помещаем в обект статусов сервиса
// fn process_pdf(path: &str, config: Arc<PdfRenderConfig>, page: Option<u32>) -> Vec<String> 
// {
//     PDF_SERVICE_STATUS.lock().unwrap().set_processing(true);
//     let images = process_internal(path, &config, page);
//     PDF_SERVICE_STATUS.lock().unwrap().set_processing(false);
//     if images.is_err()
//     {
//         let error = images.err().unwrap();
//         PDF_SERVICE_STATUS.lock().unwrap().add_message(error.to_string());
//         error!("Ошибка извлечения изображений из файла {}, {} -> {}", path, error, backtrace!());
//         return vec![];
//     }
//     return images.unwrap();
// }
// fn process_pdf_to_u8(path: &str, config: Arc<PdfRenderConfig>, page: Option<u32>) -> Vec<Vec<u8>> 
// {
//     PDF_SERVICE_STATUS.lock().unwrap().set_processing(true);
//     let images = process_internal_to_u8(path, &config, page);
//     PDF_SERVICE_STATUS.lock().unwrap().set_processing(false);
//     if images.is_err()
//     {
//         let error = images.err().unwrap();
//         PDF_SERVICE_STATUS.lock().unwrap().add_message(error.to_string());
//         error!("Ошибка извлечения изображений из файла {}, {} -> {}", path, error, backtrace!());
//         return vec![];
//     }
//     return images.unwrap();
// }


// fn thumbs(path: &str, page: Option<u32>) -> Vec<String> 
// {
//     let config = Arc::clone(&CONFIG_THUMBS);
//     process_pdf(path, config, page)  
// }
// fn document(path: &str, page: Option<u32>) -> Vec<String>
// {
//     let config = Arc::clone(&CONFIG_ALL);
//     process_pdf(path, config, page)    
// }

// // fn render<C, F: Send + 'static>(process: F, callback: C) -> Vec<String> 
// //     where C: Fn(PdfProcessingStatus),
// //     F: Fn()->Vec<String>
// // {
// //     let (sender, receiver) = channel();
// //     let s = sender.clone();
// //     std::thread::spawn(move || 
// //     {
// //         let images = process();
// //         debug!("Обработано {} страниц", images.len());
// //         s.send(images).unwrap();
// //     });
// //     update_callback(callback);
// //     drop(sender);
// //     if let Some(one_receiver) = receiver.iter().next()
// //     {
// //         return one_receiver;
// //     }
// //     else 
// //     {
// //         error!("Ошибка получения объекта от распространителя! {}", backtrace!());
// //         PDF_SERVICE_STATUS.lock().unwrap().add_message("Ошибка получения обьекта от распростанителя!");
// //         return vec![]
// //     };
// // }

// async fn render<C, F: Send + 'static>(process: F, callback: C) -> Vec<String> 
//     where C: Fn(PdfProcessingStatus),
//     F: Fn()->Vec<String> + Send + Sync
// {
//     let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
//     let s = sender.clone();
//     let p = tokio::spawn(
//     async move {
//         let images = process();
//         debug!("Обработано {} страниц", images.len());
//         s.send(images).await
//     });
//     update_callback(callback).await;
//     drop(sender);
//     if let Some(one_receiver) = receiver.recv().await
//     {
//         return one_receiver;
//     }
//     else 
//     {
//         error!("Ошибка получения объекта от распространителя!");
//         PDF_SERVICE_STATUS.lock().unwrap().add_message("Ошибка получения обьекта от распростанителя!");
//         return vec![]
//     };
// }

// // async fn update_callback<F>(callback: F) where F: Fn(PdfProcessingStatus)
// // {
// //     let dur = std::time::Duration::from_millis(1000);
// //     loop 
// //     {
// //         std::thread::sleep(dur);
// //         let current_status = PDF_SERVICE_STATUS.lock().unwrap().get_current_status();
// //         debug!("Завершено: {}%", current_status.get_percentage());
// //         callback(current_status);
// //         if !PDF_SERVICE_STATUS.lock().unwrap().is_processing()
// //         {
// //             break;
// //         }
// //     } 
// // }
// async fn update_callback<F>(callback: F) where F: Fn(PdfProcessingStatus)
// {
//     let dur = tokio::time::Duration::from_millis(1000);
//     loop 
//     {
//         tokio::time::sleep(dur).await;
//         let current_status = PDF_SERVICE_STATUS.lock().unwrap().get_current_status();
//         debug!("Завершено: {}%", current_status.get_percentage());
//         callback(current_status);
//         if !PDF_SERVICE_STATUS.lock().unwrap().is_processing()
//         {
//             break;
//         }
//     } 
// }


// //Возвращает массив изображений в формате base64
// pub async fn render_document<C>(path: &str, page: Option<u32>, callback: C) -> Vec<String> where C: Fn(PdfProcessingStatus)
// {
//     if let Some(path) = check_path(path)
//     {
//         let result = render(move ||
//         {
//             document(&path, page)
//         }, callback).await;
//         return result;
//     }
//     else 
//     {
//         return vec![];
//     }
// }

// pub async fn render_thumbnails<C>(path: &str, page: Option<u32>, callback: C) -> Vec<String> where C: Fn(PdfProcessingStatus)
// {
//     if let Some(path) = check_path(path)
//     {
//         let result = render(move ||
//         {
//             thumbs(&path, page)
//         }, callback).await;
//         return result;
//     }
//     else 
//     {
//         return vec![];
//     }
// }

// fn check_path(path: &str) -> Option<String>
// {
//     let path = Path::new(path);
//     if path.exists()
//     {
//         let p = path.to_str();
//         if p.is_none()
//         {
//             let err = format!("Ошибка, невозможно сформировать корректный путь для файла {}", path.display());
//             PDF_SERVICE_STATUS.lock().unwrap().add_message(&err);
//             error!("{}", &err);
//             return None;
//         }
//         let p = p.unwrap().to_owned();
//         Some(p)
//     }
//     else 
//     {
//         let err = format!("Ошибка, файл {} не найден", path.display());
//         PDF_SERVICE_STATUS.lock().unwrap().add_message(&err);
//         error!("{}", &err);
//         return None;    
//     } 
// }

// #[cfg(test)]
// mod tests
// {
//     use logger::debug;
//     use super::{PDF_SERVICE_STATUS, render_document, render_thumbnails};

//     #[tokio::test]
//     async fn test_document_render()
//     {
//         logger::StructLogger::new_default();
//         let path = "/hard/xar/medo_testdata/0/15933154/text0000000000.pdf";
//         let rendered_pdf = render_document(path, None, |status|
//         {
//             debug!("(ТЕСТ)Завершено: {}% Активен {}, Ошибки: {:?}", status.get_percentage(), status.is_processing(), status.get_messages());
//         }).await;
//         debug!("Обработка файла {} завершена, получено {} страниц", path, rendered_pdf.len());
//     }

//     #[test]
//     fn test_thumbs_render()
//     {
//         logger::StructLogger::new_default();
//         let path = "/hard/xar/medo_testdata/0/15933154/text0000000000.pdf";
//         render_thumbnails(path, None, |status|
//         {
//             debug!("(ТЕСТ)Завершено: {}% Активен {}, Ошибки: {:?}", status.get_percentage(), status.is_processing(), status.get_messages());
//         });
//         debug!("Обработка файла {} завершена", path);
//     }

//     #[test]
//     fn test_page_render()
//     {
//         logger::StructLogger::new_default();
//         let path = "/hard/xar/medo_testdata/0/15933154/text0000000000.pdf";
//         render_document(path, Some(22), |status|
//         {
//             debug!("(ТЕСТ)Завершено: {}% Активен {}, Ошибки: {:?}", status.get_percentage(), status.is_processing(), status.get_messages());
//         });
//         debug!("Обработка файла {} завершена", path);
//     }

//     #[tokio::test]
//     async fn test_wrong_page_render()
//     {
//         logger::StructLogger::new_default();
//         let path = "/hard/xar/medo_testdata/0/15933154/text0000000000.pdf";
//         let rendered_pdf = render_document(path, Some(23), |status|
//         {
//             debug!("(ТЕСТ)Завершено: {}% Активен {}, Ошибки: {:?}", status.get_percentage(), status.is_processing(), status.get_messages());
//         }).await;
//         debug!("Обработка файла {} завершена, получено {} страниц", path, rendered_pdf.len());
//     }
// }

// #[cfg(test)]
// mod async_tests
// {
//     use logger::debug;
//     use super::PDF_SERVICE_STATUS;
    
//     //use tokio::test;

//     async fn test_async_render()
//     {
//         logger::StructLogger::new_default();
//         let path = "/hard/xar/medo_testdata/0/15933154/text0000000000.pdf";
//         super::render_document(path, None, |status|
//         {
//             debug!("(ТЕСТ)Завершено: {}% Активен {}, Ошибки: {:?}", status.get_percentage(), status.is_processing(), status.get_messages());
//         });
//         println!("Обработка файла {} завершена", path);
//     }

//     #[tokio::test]
//     async fn test_async_render_call()
//     {
//         let call = test_async_render().await;
//         println!("Обработка файла завершена");
//     }

//     // #[tokio::test]
//     // async fn test_async_render()
//     // {
//     //     logger::StructLogger::initialize_logger();
//     //     let path = "/hard/xar/medo_testdata/0/15933154/text0000000000.pdf";
//     //     let dur = std::time::Duration::from_millis(1000);
//     //     let result = super::render_document_async(path).await;
//     //     // std::thread::spawn(move || 
//     //     // {
//     //     //     let conv = super::render_document(path);
//     //     //     debug!("Обработано {} страниц", conv.len());
//     //     // });
//     //     //эмуляция обработки процентажа
//     //     loop 
//     //     {
//     //         std::thread::sleep(dur);
//     //         debug!("Завершено: {}%", PDF_SERVICE_STATUS.lock().unwrap().get_percentage());
//     //         if !PDF_SERVICE_STATUS.lock().unwrap().in_progress()
//     //         {
//     //             break;
//     //         }
//     //     } 
//     // }
// }