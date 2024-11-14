use std::{io::Cursor, sync::Arc};
use crate::Error;

use super::error;
use image::{GrayImage, ImageFormat, RgbaImage, DynamicImage};
use logger::error;
use tokio::runtime::Handle;
use utilites::Hasher;
use pdfium_render::prelude::{PdfBitmapFormat, PdfPageRenderRotation, PdfRenderConfig, Pdfium};
//use pdfium_render::prelude::*;
pub struct PdfService 
{
    config: Arc<PdfRenderConfig>,
    path: String,
}
impl PdfService
{
    pub fn new(path: &str, w: i32, h: i32) -> Self
    {
        Self 
        { 
            config: Arc::new(PdfRenderConfig::new().set_target_width(w).set_maximum_height(h).rotate_if_landscape(PdfPageRenderRotation::Degrees90, true)),
            path: path.to_owned()
        }
    }
    fn get_instance() -> Result<Pdfium, error::Error> 
    {
        let dirs = ["libs/pdf/libs/", "pdf/libs/", "./libs/", "libs/"];
        let binding_result = 
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(dirs[0]))
        .or_else(|_| Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(dirs[1])))
        .or_else(|_| Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(dirs[2])))
        .or_else(|_| Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(dirs[3])));
        if let Ok(result) = binding_result
        {
            return Ok(Pdfium::new(result));
        }
        else
        {
            let unrecognition_err : pdfium_render::prelude::PdfiumError = pdfium_render::prelude::PdfiumError::UnrecognizedPath;
            error!("библиотека pdfium не найдена в {} -> {}", dirs.join(","), unrecognition_err.to_string());
            return Err(error::Error::PdfiumError(unrecognition_err)); 
        }
       
    }
    
    ///Извлечение изображения из pdf и выдача в формате строки base64
    pub async fn convert_pdf_page_to_image(&self, page_number: u32) -> Result<String, error::Error> 
    {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let config = Arc::clone(&self.config);
        let path = self.path.clone();
        let path2 = path.clone();
        let current = Handle::current();
        tokio::task::spawn_blocking(move || 
        {
            current.block_on(
            async move
            {
                let pdfium = Self::get_instance();
                if pdfium.is_err()
                {
                    let _ = sender.send(Err(pdfium.err().unwrap()));
                    return;
                }
                let pdfium = pdfium.unwrap();
                let document = pdfium.load_pdf_from_file(&path, None);
                if document.is_err()
                {
                    let _ = sender.send(Err(error::Error::PdfiumError(document.err().unwrap())));
                    return;
                }
                let document = document.unwrap();
                let pages_count = document.pages().len();
                if page_number < 1 || page_number > pages_count as u32
                {
                    let _ = sender.send(Err(error::Error::WrongPageSelect(path.clone(), pages_count as u32, page_number)));
                    return;
                }
                let page_index = (page_number -1) as usize;
                let page = document.pages().iter().nth(page_index).unwrap();
                let current_page =  page.render_with_config(&config);
                if current_page.is_err()
                {
                    let _ = sender.send(Err(error::Error::PdfiumError(current_page.err().unwrap())));
                    return;
                }
                let current_page = current_page.unwrap();
                let bytes = current_page.as_rgba_bytes();
                let width = current_page.width() as u32;
                let height = current_page.height() as u32;
                let image = match current_page.format().unwrap_or_default() 
                {
                    PdfBitmapFormat::BGRA
                    | PdfBitmapFormat::BRGx
                    | PdfBitmapFormat::BGRx
                    | PdfBitmapFormat::BGR => 
                    {
                        RgbaImage::from_raw(width, height, bytes).map(DynamicImage::ImageRgba8)
                    }
                    PdfBitmapFormat::Gray => 
                    {
                        GrayImage::from_raw(width, height, bytes).map(DynamicImage::ImageLuma8)
                    }
                };
                let _ = sender.send(image.ok_or(Error::ExtractDynamicImageError(path.clone(), page_number)));
            })
        });

        if let Ok(page) = receiver.await
        {
            let image = page?;
            let png = self.convert_page(image, path2, page_number).await?;
            let base64 = Hasher::from_bytes_to_base64(&png);
            return Ok(base64);
        }
        else 
        {
            return Err(error::Error::ChannelError(self.path.clone()));
        }
    }
     ///Извлечение изображения из pdf и выдача в формате строки base64
     pub async fn get_pages_count(&self) -> Result<u16, error::Error> 
     {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let path = self.path.clone();
        let current = Handle::current();
        tokio::task::spawn_blocking(move || 
        {
            current.block_on(
            async move
            {
                let pdfium = Self::get_instance();
                if pdfium.is_err()
                {
                    let _ = sender.send(Err(pdfium.err().unwrap()));
                    return;
                }
                let pdfium = pdfium.unwrap();
                let document = pdfium.load_pdf_from_file(&path, None);
                if document.is_err()
                {
                    let _ = sender.send(Err(error::Error::PdfiumError(document.err().unwrap())));
                    return;
                }
                let document = document.unwrap();
                let pages_count = document.pages().len();
                let _ = sender.send(Ok(pages_count));
            })
        });

        if let Ok(pages) = receiver.await
        {
            return pages;
        }
        else 
        {
            return Err(error::Error::ChannelError(self.path.clone()));
        }
    }

    // Извлечение страницы из pdf и преобразование ее в формат rgba8 pdf и выдача страницы в виде массива байт
    async fn convert_page(&self, dyn_image: DynamicImage, path: String, page_number: u32) -> Result<Vec<u8>, error::Error>
    {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let current = Handle::current();
        tokio::task::spawn_blocking(move || 
        {
            current.block_on(
            async move
            {
                let mut writer = std::io::BufWriter::new(Cursor::new(vec![]));
                let rgba8 = dyn_image.as_rgba8();
                if let Some(rgba) = rgba8
                {
                    let _ = rgba.write_to(&mut writer, ImageFormat::Png);
                    let flash = writer.into_inner();
                    if let Ok(f) = flash
                    {
                        let buff = f.into_inner();
                        let _ = sender.send(Ok(buff));
                    }
                    else 
                    {
                        let _ = sender.send(Err(error::Error::WriteBufferError(path.to_owned(), page_number as u32)));
                    }
                }
                else 
                {
                    let _ = sender.send(Err(error::Error::Rgba8ConvertError(path.to_owned(), page_number as u32)));
                }
            })
        });
        if let Ok(image) = receiver.await
        {
            return image;
        }
        else 
        {
            return Err(error::Error::ChannelError(self.path.clone()));
        }
    }
}


#[cfg(test)]
mod async_tests
{
    use std::future::Future;

    use futures::future::join_all;
    use logger::debug;
    use tokio::task::JoinSet;
    
    //use tokio::test;
    #[tokio::test]
    async fn test_async_render()
    {
        let _ = logger::StructLogger::new_default();
        let path = "/hard/xar/medo_testdata/0/15933154/text0000000000.pdf";
        let service = super::PdfService::new(path, 600, 800);
        debug!("main: {:?}", std::thread::current().id());
        let now = std::time::Instant::now();
        let futures: Vec<_> = (1..20).map(|i| service.convert_pdf_page_to_image(i)).collect();
        let r = join_all(futures).await;
        let lenghts = r.iter().map(|f| f.as_ref().unwrap().len()).collect::<Vec<usize>>();
        assert_eq!(&lenghts, &[194944, 230068, 227336, 229548, 243152, 240192, 227376, 244440, 223816, 213632, 219396, 251056, 249396, 231444, 240676, 251600, 274848, 245200, 216220]);
        debug!("Тестирование завершено за {}мc -> lenghts: {:?}",  now.elapsed().as_millis(), &lenghts);
    }
}