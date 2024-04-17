use std::net::SocketAddr;
use std::sync::Arc;

use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Empty, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use logger::{debug, error, info};
use serde::{Deserialize, Serialize};
use settings::{Settings, Task};
use tokio::net::{TcpListener, TcpStream};
use anyhow::{anyhow, Context, Error, Result};
use service::Server;
use transport::{BytesSerializer, Contract, NewPacketInfo};
use crate::state::AppState;
use crate::{commands, WebsocketServer, APP_STATE};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
static NOTFOUND: &[u8] = b"Not Found";

pub async fn start_http_server(port: usize) -> Result<()>
{
    let addr = ["127.0.0.1:".to_owned(), port.to_string()].concat();
    
        let addr: SocketAddr = addr.parse().unwrap();
        let listener = TcpListener::bind(&addr).await?;
        debug!("api доступно по http://{}", addr);
        tokio::spawn(async move
        {
            loop 
            {
                let connected = listener.accept().await;
                if let Ok((stream, addr)) = connected
                {
                    let io = TokioIo::new(stream);
                    tokio::task::spawn(async move 
                    {
                        let service = service_fn(move |req|
                        {
                            info!("Поступил запрос от {} headers->{:?}", &addr, req.headers());
                            response_examples(req)
                        });
                        if let Err(err) = http1::Builder::new().serve_connection(io, service).await 
                        //еще настройки из https://docs.rs/hyper/latest/hyper/server/conn/http1/struct.Builder.html
                        {
                            error!("Ошибка обслуживания соединения: {:?}", err);
                        }
                    });
                }
                else 
                {
                    error!("Ошибка подключения клиента к api {}", connected.err().unwrap().to_string());
                }
                
            }
        });
    Ok(())
}

async fn response_examples(req: Request<IncomingBody>) -> Result<Response<BoxBody>> 
{
    let app_state = Arc::clone(&APP_STATE);
    match (req.method(), req.uri().path()) 
    {
        (&Method::GET, "/settings/tasks") => get_tasks(app_state).await,
        (&Method::POST, "/settings/tasks/update") => update_task(req, app_state).await,
        (&Method::POST, "/settings/tasks/delete") => delete_task(req, app_state).await,
        (&Method::GET, "packets/truncate") => truncate(app_state).await,
        (&Method::GET, "packets/clean") => clean(app_state).await,
        (&Method::POST, "packets/rescan") => rescan(req, app_state).await,
        _ => 
        {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full(NOTFOUND))
                .unwrap())
        }
    }
}
//type GenericError = Box<dyn std::error::Error + Send + Sync>;
//type Result<T> = std::result::Result<T, GenericError>;

// static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
// static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

// static POST_DATA: &str = r#"{"original": "data"}"#;
// static URL: &str = "http://127.0.0.1:1337/json_api";

// async fn client_request_response() -> Result<Response<BoxBody>> {
//     let req = Request::builder()
//         .method(Method::POST)
//         .uri(URL)
//         .header(header::CONTENT_TYPE, "application/json")
//         .body(Full::new(Bytes::from(POST_DATA)))
//         .unwrap();

//     let host = req.uri().host().expect("uri has no host");
//     let port = req.uri().port_u16().expect("uri has no port");
//     let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
//     let io = TokioIo::new(stream);

//     let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

//     tokio::task::spawn(async move {
//         if let Err(err) = conn.await {
//             println!("Connection error: {:?}", err);
//         }
//     });

//     let web_res = sender.send_request(req).await?;

//     let res_body = web_res.into_body().boxed();

//     Ok(Response::new(res_body))
// }

// async fn api_post_response(req: Request<IncomingBody>) -> Result<Response<BoxBody>> 
// {
//     // Aggregate the body...
//     let whole_body = req.collect().await?.aggregate();
//     // Decode as JSON...
//     let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
//     // Change the JSON...
//     data["test"] = serde_json::Value::from("test_value");
//     // And respond with the new JSON.
//     let json = serde_json::to_string(&data)?;
//     let response = Response::builder()
//         .status(StatusCode::OK)
//         .header(header::CONTENT_TYPE, "application/json")
//         .body(full(json))?;
//     Ok(response)
// }

//получается это единственная фунция что нужна?) а может и вообще ненужна если норм реализовать ящик Contract
async fn get_tasks(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    // let data = 
    // {
    //     let guard = app_state.settings.lock().await;
    //     guard.clone()
    // };
    let settings = commands::settings::get(app_state).await?;
    let bytes = settings.to_bytes()?;
    let body_data = to_body(bytes);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .body(body_data)?;
    Ok(response)
}

async fn update_task(req: Request<IncomingBody>, app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let body = req.collect().await?.to_bytes();
    let task: Task = Task::from_bytes(&body)?;
    if let Err(e) = commands::settings::update(task.clone(), app_state).await
    {
        return error_responce(e);
    }
    let response = response_ok_empty()?;
    //сообщаем всем через вебсокет что мы обновили какую то таску
    WebsocketServer::task_update_event(task).await;
    Ok(response)
}

async fn delete_task(req: Request<IncomingBody>, app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let body = req.collect().await?.to_bytes();
    let task: Task = Task::from_bytes(&body)?;
    if let Err(e) = commands::settings::delete(task.clone(), app_state).await
    {
        return error_responce(e);
    }
    let response = response_ok_empty()?;
    WebsocketServer::task_delete_event(task).await;
    Ok(response)
}

async fn clean(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    if let Err(e) = commands::service::clear_dirs(app_state).await
    {
        return error_responce(e);
    }
    let response = response_ok_empty()?;
    Ok(response)
}
async fn truncate(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    if let Err(e) = commands::service::truncate_tasks_excepts(app_state).await
    {
        return error_responce(e);
    }
    let response : Response<BoxBody> = response_ok_empty()?;
    Ok(response)
}

async fn rescan(req: Request<IncomingBody>, app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let body = req.collect().await?.to_bytes();
    let packet = NewPacketInfo::from_bytes(&body)?;
    if let Err(e) = commands::service::rescan_packet(packet, app_state).await
    {
        return error_responce(e);
    }
    let response = response_ok_empty()?;
    Ok(response)
}

// async fn api_get_response() -> Result<Response<BoxBody>> 
// {
//     let data = vec!["foo", "bar"];
//     let res = match serde_json::to_string(&data) 
//     {
//         Ok(json) => Response::builder()
//             .header(header::CONTENT_TYPE, "application/json")
//             .body(full(json))
//             .unwrap(),
//         Err(_) => Response::builder()
//             .status(StatusCode::INTERNAL_SERVER_ERROR)
//             .body(full(INTERNAL_SERVER_ERROR))
//             .unwrap(),
//     };
//     Ok(res)
// }



// let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
//             Ok(Response::new(full(reversed_body)))

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody 
{
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn response_ok<T : BytesSerializer + Serialize>(obj: T) -> Result<Response<BoxBody>>
{
    
    let bytes = obj.to_bytes()?;
    let body_data = to_body(bytes);
    let resp = Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/octet-stream")
    .body(body_data)?;
    Ok(resp)
}

fn response_ok_empty() -> Result<Response<BoxBody>>
{
    let resp =  Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/json")
    .body(empty_body())?;
    Ok(resp)
    
}

pub fn to_body(bytes: Bytes) -> BoxBody
{
    Full::new(bytes)
        .map_err(|never| match never {})
        .boxed()
}

fn empty_body() -> BoxBody
{
    to_body(Bytes::new())
}

fn error_responce(error: crate::Error) -> Result<Response<BoxBody>>
{
    let req = Response::builder()
        .status(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(full(error.to_string()))?;
    Ok(req)
}

// pub trait ToBody
// {
//     fn to_body(&self) -> Result<BoxBody> where Self: Serialize
//     {
//         let mut s = flexbuffers::FlexbufferSerializer::new();
//         let _ = self.serialize(&mut s)?;
//         let bytes = Bytes::copy_from_slice(s.view());
//         Ok(Full::new(bytes)
//             .map_err(|never| match never {})
//             .boxed())
//     }
//     fn from_body(body: &Bytes) -> anyhow::Result<Self> where for <'de> Self : Deserialize<'de>
//     {
//         let r = flexbuffers::Reader::get_root(body.as_ref())?;
//         let deserialize = Self::deserialize(r).with_context(|| "Ошибка десериализации".to_owned())?;
//         Ok(deserialize)
//     }
// }
// impl ToBody for Settings{}
// impl ToBody for Task{}


