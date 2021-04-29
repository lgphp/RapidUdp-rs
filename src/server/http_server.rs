#![allow(dead_code, unused_imports, non_snake_case, unused_imports, unused_variables, unused_must_use)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Request, Body, Response, Method, StatusCode, Server as HttpServer};
use serde::Serialize;
use crate::server::udp_server::Server;
use hyper::header::CONTENT_TYPE;
use bytes::Buf;
use crate::servermgr::UDP_LIST;
use log::{info, warn};
use std::collections::HashMap;
use log_mdc;


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

#[derive(Debug, Serialize)]
struct ResJson<T> where T:Sized + Serialize{
    code: i16,
    message: String,
    data: T,
}

pub struct HttpConfig {
    addr: String,
}

static NOTFOUND: &[u8] = b"Not Found";

impl HttpConfig {
    pub fn new(addr: String) -> HttpConfig {
        return HttpConfig {
            addr
        };
    }
}

pub async fn http_start(c: HttpConfig) -> Result<()> {
    let addr = &c.addr.as_str().parse().expect("http 服务地址不合法");
    let service = make_service_fn(move |_| {
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                route(req)
            }))
        }
    });
    let server = HttpServer::bind(addr).serve(service);
    info!("Web服务启动 on http://{}", addr);
    server.await?;
    Ok(())
}


pub async fn route(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/udp/bindUdp") => bindUdp(req).await,
        (&Method::GET, "/udp/monitor") => monitor(req).await,
        (&Method::GET, "/udp/pingUdp") => pingUdp(req).await,
        _ => {
            let data = ResJson {
                code: 0,
                message: "接口无法找到!".to_string(),
                data: (),
            };
            let json = serde_json::to_string(&data);
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json.unwrap()))
                .unwrap())
        }
    }
}

/// 接收udpmaster的指令
///
pub async fn bindUdp(req: Request<Body>) -> Result<Response<Body>> {
    log_mdc::insert("requestId" , "1929931231321231");
    let data = ResJson {
        code: 0,
        message: "".to_string(),
        data: (),
    };
    let whole_body = hyper::body::aggregate(req).await?;
    let req_body: serde_json::Value = serde_json::from_reader(whole_body.reader()).unwrap();
    let selfAddr = req_body.get("selfAddr").unwrap().as_str().unwrap().to_string();
    let leftAddr = req_body.get("leftAddr").unwrap().as_str().unwrap().to_string();
    let rightAddr = req_body.get("rightAddr").unwrap().as_str().unwrap().to_string();
    start_udp_server(leftAddr , selfAddr , rightAddr);
    let json = serde_json::to_string(&data);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json.unwrap()))
        .unwrap())
}

///
/// 启动一个udp服务器
///
fn start_udp_server(leftAddr :String ,selfAddr:String, rightAddr :String) {
    let mut running = false;
    unsafe {
        if let Some(ref   sm) = UDP_LIST{
            let en  = sm.entries();
            let running_servers = en.iter().map(|s|{
                s.0.as_str()
            }).filter(|k|->bool {*k==selfAddr.as_str()}).collect::<Vec<_>>();
            if running_servers.len() > 0 {
                info!("{} # 已经启动了....不能重复启动", selfAddr);
                running = true;
            }
        }
    }
    if !running {
        tokio::task::spawn({
            async move {
                let mut udpserver = Server::new(leftAddr,
                                                selfAddr,
                                                rightAddr);
                udpserver.start().await;
            }
        });
    }
}

pub async fn pingUdp(_req:Request<Body>) ->Result<Response<Body>>{
    let data = ResJson {
        code: 0,
        message: "".to_string(),
        data: (),
    };
    let json = serde_json::to_string(&data);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json.unwrap()))
        .unwrap())
}

///
/// 监控udpserver运行情况
///
pub async fn monitor(_req: Request<Body>) -> Result<Response<Body>> {
    let mut udp_map  = HashMap::<String, i64>::new();
    unsafe {
        if let Some(ref m) = &UDP_LIST {
             m.entries().iter().for_each( |k |  {
                 udp_map.insert(k.0.clone(), k.1);
            } );
        }
    }
    let data = ResJson {
        code: 200,
        message: "".to_string(),
        data: udp_map,
    };
    let json = serde_json::to_string(&data);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json.unwrap()))
        .unwrap())
}
