use std::{io::Read, net::SocketAddr, str::FromStr, sync::Arc};
use http_body_util::{BodyExt, Empty, Full};
use hyper::StatusCode;
use hyper_util::rt::TokioIo;
use logger::{debug, error};
use serde::{Deserialize, Serialize};
use service::Client;
use settings::Task;
use transport::{File, FileRequest, FilesRequest, Packet, Pagination};
use tokio::net::TcpStream;
use crate::{ws_serivice::WebsocketClient, Error};
use utilites::http::{Bytes, HyperClient, HeaderName, ACCEPT, USER_AGENT};
type Result<T> = anyhow::Result<T, Error>;


fn get_client(api_path: &str) -> HyperClient
{
    HyperClient::new_with_timeout(api_path.parse().unwrap(), 850, 1200, 12).with_headers(headers())
}

fn headers() -> Vec<(HeaderName, String)>
{
    let mut h= Vec::new();
    h.push((USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0".to_owned()));
    h.push((ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_owned()));
    h
}
 ///Проверка что пришел код 200 на запрос
fn code_error_check(response: (StatusCode, Bytes), need_code: u16) -> Result<Bytes>
{
    if response.0 != utilites::http::StatusCode::OK
    {
        
        if let Ok(body) = String::from_utf8(response.1.to_vec())
        {
            error!("{:?}", &body);
            return Err(Error::StatusCodeError(need_code, response.0.as_u16(), Some(body)));
        }
        else
        {
            return Err(Error::StatusCodeError(need_code, response.0.as_u16(), None));
        }
    }
    else 
    {
        Ok(response.1)
    }
}

async fn get<T>(api_path: &str, uri_path: &str) -> Result<T> where T: for <'de> Deserialize<'de>
{
    let client = get_client(api_path);
    let client = client.add_path(uri_path);
    let result = client.get().await?;
    let result = code_error_check(result, 200)?;
    let result = serde_json::from_slice::<T>(&result)?;
    Ok(result)
}
async fn get_with_body<I: Serialize + Clone, O>(api_path: &str, uri_path: &str, body: I) -> Result<O> where O: for <'de> Deserialize<'de>
{
    let client = get_client(api_path);
    let client = client.add_path(uri_path);
    let result = client.get_with_body(body).await?;
    let result = code_error_check(result, 200)?;
    let result = serde_json::from_slice::<O>(&result)?;
    Ok(result)
}
async fn post<B: Serialize + Clone>(payload: B, api_path: &str, uri_path: &str) -> Result<()>
{
    let client = get_client(api_path);
    let client = client.add_path(uri_path);
    let upd = client.post_with_body(payload).await?;
    let _ = code_error_check(upd, 200)?;
    Ok(())
}
async fn patch<B: Serialize + Clone>(payload: B, api_path: &str, uri_path: &str) -> Result<()>
{
    let client = get_client(api_path);
    let client = client.add_path(uri_path);
    let upd = client.patch_with_body(payload).await?;
    let _ = code_error_check(upd, 200)?;
    Ok(())
}
async fn put<B: Serialize + Clone>(payload: B, api_path: &str, uri_path: &str) -> Result<()>
{
    let client = get_client(api_path);
    let client = client.add_path(uri_path);
    let upd = client.put_with_body(payload).await?;
    let _ = code_error_check(upd, 200)?;
    Ok(())
}

async fn delete(api_path: &str, uri_path: &str, params: &[(&str, &str)]) -> Result<()>
{
    let client = get_client(api_path);
    let client = client.add_path(uri_path);
    let result = client.delete(params).await?;
    let _ = code_error_check(result, 200)?;
    Ok(())
}



pub struct SettingsService
{
    api_path: String
}
impl SettingsService
{
    pub fn new(path: &str) -> SettingsService
    {
        SettingsService { api_path: path.to_owned() }
    }
    pub async fn get(&self) -> Result<Vec<Task>>
    {
        let result = get(&self.api_path, "settings/tasks").await?;
        Ok(result)
    }
    pub async fn update(&self, payload: Task) -> Result<()>
    {
        put(payload, &self.api_path, "settings/tasks/update").await?;
        Ok(())
    }
    pub async fn delete(&self, payload: Task) -> Result<()>
    {
        let _ = delete(&self.api_path, "settings/tasks/delete", &[("name", payload.get_task_name())]).await?;
        Ok(())
    }
}


pub struct UtilitesService
{
    api_path: String
}
impl UtilitesService
{
    pub fn new(path: &str) -> UtilitesService
    {
        UtilitesService { api_path: path.to_owned() }
    }
    pub async fn clear_dirs(&self)
    {
        WebsocketClient::send_message(transport::Contract::CleanStart).await;
    }

    pub async fn truncate_tasks_excepts(&self) -> Result<u32>
    {
        let result = get(&self.api_path, "packets/truncate").await?;
        Ok(result)
    }

    pub async fn rescan_packet(&self, payload: Packet) -> Result<()>
    {
        post(payload, &self.api_path, "packets/rescan").await?;
        Ok(())
    }

    pub async fn delete_packet(&self, payload: Packet) -> Result<()>
    {
        post(payload, &self.api_path, "packets/delete").await?;
        Ok(())
    }

}

pub struct PacketService
{
    api_path: String
}

#[derive(Serialize, Clone)]
pub struct SearchingValue<'a>
{
    value: &'a str
}
impl PacketService
{
    pub fn new(path: &str) -> Self
    {
        Self { api_path: path.to_owned() }
    }
    pub async fn get(&self, Pagination {row, offset} : Pagination) -> Result<Vec<Packet>>
    {
        let client = get_client(&self.api_path);
        let client = client.add_path("packets");
        let result = client.get_with_params(&[("limit".to_owned(), row.to_string()), ("offset".to_owned(), offset.to_string())]).await?;
        let result = code_error_check(result, 200)?;
        let result = serde_json::from_slice::<Vec<Packet>>(&result)?;
        Ok(result)
    }
    pub async fn search(&self, payload: &str) -> Result<Vec<Packet>>
    {
        let client = get_client(&self.api_path);
        let client = client.add_path("packets/search");
        let result = client.get_with_body(SearchingValue {value: payload}).await?;
        let result = code_error_check(result, 200)?;
        let result = serde_json::from_slice::<Vec<Packet>>(&result)?;
        Ok(result)
    }
    pub async fn count(&self) -> Result<u32>
    {
        let result = get(&self.api_path, "packets/count").await?;
        Ok(result)
    }
    pub async fn get_files_list(&self, FilesRequest {task_name, dir_name} : FilesRequest) -> Result<Vec<File>>
    {
        let result = get_with_body(&self.api_path, "packets/files", FilesRequest {task_name, dir_name}).await?;
        Ok(result)
    }   
    pub async fn get_pdf_pages_count(&self, FileRequest { file: File {file_name, file_type, path}, page_number }: FileRequest) -> Result<u16>
    {
        let result = get_with_body(&self.api_path, "packets/pdf/pages", FileRequest { file: File {file_name, file_type, path}, page_number }).await?;
        Ok(result)
    }   
    pub async fn get_pdf_page(&self, FileRequest { file: File {file_name, file_type, path}, page_number }: FileRequest) -> Result<String>
    {
        let result = get_with_body(&self.api_path, "packets/pdf", FileRequest { file: File {file_name, file_type, path}, page_number }).await?;
        Ok(result)
    }   
    pub async fn get_file_body(&self, FileRequest { file: File {file_name, file_type, path}, page_number }: FileRequest) -> Result<String>
    {
        let result = get_with_body(&self.api_path, "packets/file", FileRequest { file: File {file_name, file_type, path}, page_number: None }).await?;
        Ok(result)
    }   
}



#[cfg(test)]
mod tests
{
    // use bytes::Bytes;
    // use http_body_util::Empty;
    // use hyper::Request;
    // use logger::{debug, StructLogger};

    // #[test]
    // fn test_headers()
    // {
    //     StructLogger::new_default();
    //     let req_path = ["http://127.0.0.1:3010/", "users"].concat();
    //     let uri = req_path.parse::<hyper::Uri>().unwrap();
    //     let authority = uri.authority().unwrap().clone();
    //     let req = Request::builder()
    //     .method("POST")
    //     .uri(uri)
    //     .header(hyper::header::HOST, authority.as_str())
    //     .header(hyper::header::CONNECTION, "keep-alive")
    //     .header("Keep-Alive", "timeout=5, max=50")
    //     .body(Empty::<Bytes>::new());
    //     debug!("{:?}", req.unwrap());
    // }
}