use std::{io::Read, net::SocketAddr, str::FromStr};
use http_body_util::{BodyExt, Empty, Full};
use hyper::{Request, StatusCode};
use hyper_util::{client::legacy::Client, rt::TokioIo};
use logger::error;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use transport::BytesSerializer;
use tokio::net::TcpStream;
use bytes::Bytes;
use crate::Error;

type Result<T> = anyhow::Result<T, Error>;
static HOST : OnceCell<(SocketAddr, String)> = OnceCell::new();
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
pub fn initialize_http_requests(host: String)
{
    let h: SocketAddr = host.parse().unwrap();
    HOST.set((h, ["http://", &host, "/"].concat()));
}

pub async fn get<R>(subpath: &str) -> Result<R> where for <'de> R : Deserialize<'de> + BytesSerializer
{
    let (addr, uri) = HOST.get().unwrap();
    let req_path = [uri, subpath].concat();
    let uri = req_path.parse::<hyper::Uri>().unwrap();
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move 
    {
        if let Err(err) = conn.await 
        {
            error!("Ошибка соединения с сервером: {:?}", err);
        }
    });
    let authority = uri.authority().unwrap().clone();
    let req = Request::builder()
    .uri(uri)
    .header(hyper::header::HOST, authority.as_str())
    .body(Empty::<Bytes>::new())?;
    let res = sender.send_request(req).await?;
    let status = res.status();
    let body = res.collect().await?.to_bytes();
    if status != StatusCode::OK
    {
        let dd: &[u8] = &body;
        let str = String::from_utf8(dd.to_vec()).unwrap();
        let err_format = format!("Ошибка {} -> {}", status, str);
        error!{"{}", &err_format};
        return Err(Error::RequestError(err_format));
    }
    
    let obj = R::from_bytes(&body)?;
    Ok(obj)
}

pub async fn post<R: Serialize + BytesSerializer>(subpath: &str, obj: &R) -> Result<()>
{
    let (addr, uri) = HOST.get().unwrap();
    let req_path = [uri, subpath].concat();
    let uri = req_path.parse::<hyper::Uri>().unwrap();
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    
    tokio::task::spawn(async move 
    {
        if let Err(err) = conn.await 
        {
            error!("Ошибка соединения с сервером: {:?}", &err);
        }
    });
    let authority = uri.authority().unwrap().clone();
    let bytes = obj.to_bytes()?;
    let req = Request::builder()
    .method("POST")
    .uri(uri)
    .header(hyper::header::HOST, authority.as_str())
    .header(hyper::header::CONNECTION, "keep-alive")
    .header("Keep-Alive", "timeout=5, max=50")
    .body(to_body(bytes));
    if req.is_err()
    {
        error!("{:?}", req.as_ref().err().unwrap());
    }
    sender.ready().await?;
    let res = sender.send_request(req.unwrap()).await;
    if res.is_err()
    {
        let e = format!("Ошибка post запроса для: {} -> {}", &req_path, res.err().as_ref().unwrap().to_string());
        error!("{}", &e);
        return Err(Error::RequestError(e));
    }
    if res.as_ref().unwrap().status() != StatusCode::OK
    {
        let body = res.unwrap().collect().await?.to_bytes();
        let obj = std::str::from_utf8(body.as_ref()).unwrap_or("От сервера возвращена неизвестная ошибка");
        let e = format!("Ошибка post запроса для: {} -> {}", &req_path, obj);
        error!("{}", &e);
        return Err(Error::RequestError(obj.to_owned()));
    }
    else
    {
        Ok(())
    }
}

pub fn to_body(bytes: Bytes) -> BoxBody
{
    Full::new(bytes)
        .map_err(|never| match never {})
        .boxed()
}


#[cfg(test)]
mod tests
{
    use bytes::Bytes;
    use http_body_util::Empty;
    use hyper::Request;
    use logger::{debug, StructLogger};

    #[test]
    fn test_headers()
    {
        StructLogger::initialize_logger();
        let req_path = ["http://127.0.0.1:3010/", "users"].concat();
        let uri = req_path.parse::<hyper::Uri>().unwrap();
        let authority = uri.authority().unwrap().clone();
        let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header(hyper::header::HOST, authority.as_str())
        .header(hyper::header::CONNECTION, "keep-alive")
        .header("Keep-Alive", "timeout=5, max=50")
        .body(Empty::<Bytes>::new());
        debug!("{:?}", req.unwrap());
    }
}