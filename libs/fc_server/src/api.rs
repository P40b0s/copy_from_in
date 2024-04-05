use std::net::SocketAddr;
use std::sync::Arc;

use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Empty, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use logger::{debug, error};
use serde::{Deserialize, Serialize};
use settings::{Settings, Task};
use tokio::net::{TcpListener, TcpStream};
use anyhow::{anyhow, Context, Result};
use crate::BytesSerializer;
use crate::state::AppState;
use crate::{commands, APP_STATE};

//type GenericError = Box<dyn std::error::Error + Send + Sync>;
//type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

// static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
// static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
 static NOTFOUND: &[u8] = b"Not Found";
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

async fn get_tasks(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let data = 
    {
        let guard = app_state.settings.lock().await;
        guard.clone()
    };
    let bytes = data.tasks.to_bytes()?;
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
    let _ = commands::settings::update(task, app_state).await?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(empty_body())?;
    Ok(response)
}

async fn delete_task(req: Request<IncomingBody>, app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let body = req.collect().await?.to_bytes();
    let task: Task = Task::from_bytes(&body)?;
    let _ = commands::settings::delete(task, app_state).await?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(empty_body())?;
    Ok(response)
}

async fn clear_tasks(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let _ = commands::service::clear_dirs(app_state).await?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(empty_body())?;
    Ok(response)
}
async fn truncate_tasks(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let _ = commands::service::truncate_tasks_excepts(app_state).await?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(empty_body())?;
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

async fn response_examples(req: Request<IncomingBody>) -> Result<Response<BoxBody>> 
{
    let app_state = Arc::clone(&APP_STATE);
    match (req.method(), req.uri().path()) 
    {
        (&Method::GET, "/settings/tasks") => get_tasks(app_state).await,
        (&Method::GET, "/settings/tasks/update") => update_task(req, app_state).await,
        (&Method::POST, "/settings/tasks/delete") => delete_task(req, app_state).await,
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

// let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
//             Ok(Response::new(full(reversed_body)))

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody 
{
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
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
                if let Ok((stream, _)) = connected
                {
                    let io = TokioIo::new(stream);
                    tokio::task::spawn(async move 
                    {
                        let service = service_fn(move |req| response_examples(req));
                        if let Err(err) = http1::Builder::new().serve_connection(io, service).await 
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