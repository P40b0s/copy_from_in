
// #[derive(Clone)]
// pub struct PdfProcessingStatus
// {
//     is_running: bool,
//     current_page: u32,
//     overall: u32,
//     message: Vec<String>,
//     percentage: u32,
//     pages: u32
// }

// impl Default for PdfProcessingStatus
// {
//     fn default() -> Self 
//     {
//         PdfProcessingStatus 
//         { 
//             is_running: false,
//             current_page: 0,
//             overall: 0,
//             message: vec![],
//             percentage: 0,
//             pages: 0
//         }
//     }
// }

// impl PdfProcessingStatus
// {
//     /// (c.finished / c.total) * 100
//     pub fn get_percentage(&self) -> u32
//     {
//         self.percentage
//     }
//     pub fn get_current_status(&self) -> Self
//     {
//         self.clone()
//     }
//     pub fn is_processing(&self) -> bool
//     {
//         self.is_running
//     }
//     pub fn set_pages(&mut self, pages: u32)
//     {
//         self.pages = pages;
//     }
//     pub fn get_pages(&self) -> u32
//     {
//         self.pages
//     }
//     pub fn set_percentage(&mut self, current_index: u32, overall: u32)
//     {
//         self.current_page = current_index;
//         self.overall = overall;
//         if overall > 0
//         {
//             let percentage = (self.current_page as f32 / self.overall as f32) * 100 as f32;
//             self.percentage = percentage.round() as u32;
//         }
//     }
//     pub fn add_message<M>(&mut self, msg: M) where M: Into<String>
//     {
//         self.message.push(msg.into());
//     }
//     pub fn get_messages(&self) -> Vec<String>
//     {
//         self.message.clone()
//     }
//     pub fn set_processing(&mut self, run: bool)
//     {
//         self.is_running = run;
//         if run
//         {
//             self.message = vec![];
//             self.percentage = 0;
//         }
//     }
// }