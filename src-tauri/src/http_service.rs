use std::{io::Read, net::SocketAddr, str::FromStr, sync::Arc};
use http_body_util::{BodyExt, Empty, Full};
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use logger::error;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use settings::Task;
use transport::{BytesSerializer, Packet, Pagination};
use tokio::net::TcpStream;
use crate::Error;
use utilites::http::{Bytes, HyperClient, HeaderName, ACCEPT, USER_AGENT};
type Result<T> = anyhow::Result<T, Error>;


fn get_client(api_path: &str) -> HyperClient
{
    HyperClient::new_with_timeout(api_path.parse().unwrap(), 150, 1000, 12).with_headers(headers())
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
        //let e = ["Сервер ответил кодом ", response.0.as_str(), " ожидался код ", ].concat();
        //logger::warn!("{}", &e);
        return Err(Error::StatusCodeError(need_code, response.0.as_u16()));
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
async fn post<B: Serialize + Clone>(payload: B, api_path: &str, uri_path: &str) -> Result<()>
{
    let client = get_client(api_path);
    let client = client.add_path(uri_path);
    let upd = client.post_with_body(payload).await?;
    let _ = code_error_check(upd, 200)?;
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
        // let client = get_client(&self.api_path);
        // let client = client.add_path("settings/tasks");
        // let tasks = client.get().await?;
        // let tasks = code_error_check(tasks, 200)?;
        // let tasks = serde_json::from_slice::<Vec<Task>>(&tasks)?;
        // Ok(tasks)
    }
    pub async fn update(&self, payload: Task) -> Result<()>
    {
        // let client = get_client(&self.api_path);
        // let client = client.add_path("settings/tasks/update");
        // let upd = client.post_with_body(payload).await?;
        // let _ = code_error_check(upd, 200)?;
        post(payload, &self.api_path, "settings/tasks/update").await?;
        Ok(())
    }
    pub async fn delete(&self, payload: Task) -> Result<()>
    {
        // let client = get_client(&self.api_path);
        // let client = client.add_path("settings/tasks/delete");
        // let del = client.post_with_body(payload).await?;
        // let _ = code_error_check(del, 200)?;
        post(payload, &self.api_path, "settings/tasks/delete").await?;
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
    pub async fn clear_dirs(&self) -> Result<u32>
    {
        // let client = get_client(&self.api_path);
        // let client = client.add_path("packets/clean");
        // let tasks = client.get().await?;
        // let tasks = code_error_check(tasks, 200)?;
        // let tasks = serde_json::from_slice::<u32>(&tasks)?;
        // Ok(tasks)
        let result = get(&self.api_path, "packets/clean").await?;
        Ok(result)
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

}

pub struct PacketService
{
    api_path: String
}
impl PacketService
{
    pub fn new(path: &str) -> Self
    {
        Self { api_path: path.to_owned() }
    }
    pub async fn get(&self, Pagination {row, offset} : Pagination) -> Result<Vec<Packet>>
    {
        logger::info!("pagination row:{} offset:{}", row,offset);
        let client = get_client(&self.api_path);
        let client = client.add_path("packets");
        let result = client.get_with_params(&[("limit".to_owned(), row.to_string()), ("offset".to_owned(), offset.to_string())]).await?;
        let result = code_error_check(result, 200)?;
        let result = serde_json::from_slice::<Vec<Packet>>(&result)?;
        Ok(result)
    }
    pub async fn count(&self) -> Result<u32>
    {
        // let client = get_client(&self.api_path);
        // let client = client.add_path("packets/count");
        // let result = client.get().await?;
        // let result = code_error_check(result, 200)?;
        // let result = serde_json::from_slice::<u32>(&result)?;
        let result = get(&self.api_path, "packets/count").await?;
        Ok(result)
    }

    // pub async fn get_packets_list2() -> Result<Vec<Packet>, Error>
    // {
    //     http_service::get::<Vec<Packet>>("packets/list").await
    // }
}









// pub async fn get<R>(subpath: &str) -> Result<R> where for <'de> R : Deserialize<'de> + BytesSerializer
// {
//     let (addr, uri) = HOST.get().unwrap();
//     let req_path = [uri, subpath].concat();
//     let uri = req_path.parse::<hyper::Uri>().unwrap();
//     let stream = TcpStream::connect(addr).await?;
//     let io = TokioIo::new(stream);
//     let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
//     tokio::task::spawn(async move 
//     {
//         if let Err(err) = conn.await 
//         {
//             error!("Ошибка соединения с сервером: {:?}", err);
//         }
//     });
//     let authority = uri.authority().unwrap().clone();
//     let req = Request::builder()
//     .uri(uri)
//     .header(hyper::header::HOST, authority.as_str())
//     .body(Empty::<Bytes>::new())?;
//     let res = sender.send_request(req).await?;
//     let status = res.status();
//     let body = res.collect().await?.to_bytes();
//     if status != StatusCode::OK
//     {
//         let dd: &[u8] = &body;
//         let str = String::from_utf8(dd.to_vec()).unwrap();
//         let err_format = format!("Ошибка {} -> {}", status, str);
//         error!{"{}", &err_format};
//         return Err(Error::RequestError(err_format));
//     }
//     let obj = R::from_bytes(&body)?;
//     Ok(obj)
// }

// pub async fn post<R: Serialize + BytesSerializer>(subpath: &str, obj: &R) -> Result<()>
// {
//     let (addr, uri) = HOST.get().unwrap();
//     let req_path = [uri, subpath].concat();
//     let uri = req_path.parse::<hyper::Uri>().unwrap();
//     let stream = TcpStream::connect(addr).await?;
//     let io = TokioIo::new(stream);
//     let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    
//     tokio::task::spawn(async move 
//     {
//         if let Err(err) = conn.await 
//         {
//             error!("Ошибка соединения с сервером: {:?}", &err);
//         }
//     });
//     let authority = uri.authority().unwrap().clone();
//     let bytes = obj.to_bytes()?;
//     let req = Request::builder()
//     .method("POST")
//     .uri(uri)
//     .header(hyper::header::HOST, authority.as_str())
//     .header(hyper::header::CONNECTION, "keep-alive")
//     .header("Keep-Alive", "timeout=5, max=50")
//     .body(to_body(bytes));
//     if req.is_err()
//     {
//         error!("{:?}", req.as_ref().err().unwrap());
//     }
//     sender.ready().await?;
//     let res = sender.send_request(req.unwrap()).await;
//     if res.is_err()
//     {
//         let e = format!("Ошибка post запроса для: {} -> {}", &req_path, res.err().as_ref().unwrap().to_string());
//         error!("{}", &e);
//         return Err(Error::RequestError(e));
//     }
//     if res.as_ref().unwrap().status() != StatusCode::OK
//     {
//         let body = res.unwrap().collect().await?.to_bytes();
//         let obj = std::str::from_utf8(body.as_ref()).unwrap_or("От сервера возвращена неизвестная ошибка");
//         let e = format!("Ошибка post запроса для: {} -> {}", &req_path, obj);
//         error!("{}", &e);
//         return Err(Error::RequestError(obj.to_owned()));
//     }
//     else
//     {
//         Ok(())
//     }
// }

// pub fn to_body(bytes: Bytes) -> BoxBody
// {
//     Full::new(bytes)
//         .map_err(|never| match never {})
//         .boxed()
// }


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