mod status;
mod error;
mod service;
//TODO потом переделать сервис для полного преобразования pdf и сделать его асинхронным по аналогии с постраничным сервисом
mod multiple_pages;
pub use service::PdfService;
pub use error::Error;