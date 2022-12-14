#![feature(async_closure)]
#[macro_use] extern crate log;
extern crate simplelog;
mod gnmi;
mod gnmi_ext;

use tonic::{transport::Server, Request, Response, Status};
use crate::gnmi::{CapabilityRequest, CapabilityResponse, GetRequest, GetResponse, Path, PathElem, SetRequest, SetResponse, Update};
use crate::gnmi::g_nmi_server::{GNmi, GNmiServer};
// use std::thread;
use std::string::ToString;
use std::error::Error;
use std::{env, thread};
use std::borrow::Borrow;
use std::fs::File;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::sleep;
use std::time::{Duration, Instant};
use log::{debug, error, info, LevelFilter, warn};
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};
use crate::gnmi::g_nmi_client::GNmiClient;
extern crate r2d2_redis;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use r2d2_redis::redis::{Client, Commands, RedisResult};
use std::collections::HashMap;
use r2d2_redis::r2d2::Pool;
use tonic::codegen::ok;

const DEFAULT_GNMI_SERVER_HOST_AND_PORT: &str = "[::]:8080";
const TEXT_GNMI_SERVER_HOST_AND_PORT: &'static str = "GNMI_SERVER_HOST_AND_PORT";

#[tokio::main(flavor = "multi_thread", worker_threads = 2)] //
async fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("maya_gnmi_client.log").unwrap()),
        ]
    ).unwrap();

    let gnmi_url =      match env::var_os(TEXT_GNMI_SERVER_HOST_AND_PORT) {
            Some(v) => v.into_string().unwrap().to_string(),
            None => DEFAULT_GNMI_SERVER_HOST_AND_PORT.to_string(),
        };
    let gnmi_service = GnmiService::new();
    Server::builder().add_service(GNmiServer::new(gnmi_service))
        .serve(gnmi_url.parse().unwrap())
        .await?;

    Ok(())
}
// #[derive(Debug, Default)]
pub struct GnmiService {
    pub cnt:AtomicU64,
    pub cnt_period: AtomicU64,
    pub  start: Instant,
}

#[tonic::async_trait]
impl GNmi for GnmiService {
    async fn capabilities(&self, request: Request<CapabilityRequest>) -> Result<Response<CapabilityResponse>, Status> {
        self.stats();
        let _ = request.into_inner();
        return Ok(Response::new(CapabilityResponse {
            supported_models: vec![],
            supported_encodings: vec![],
            g_nmi_version: "0.8.0".to_string(),
            extension: vec![],
        }));
    }
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        self.stats();
        todo!()
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        self.stats();
        let _ = request.into_inner();
        return Ok(Response::new(SetResponse {
            prefix: None,
            response: vec![],
            message: None,
            timestamp: 0,
            extension: vec![],
        }));
    }

    fn new() -> Self {
        GnmiService {
            start: Instant::now(),
            cnt: AtomicU64::new(0),
            cnt_period: AtomicU64::new(0)
        }
    }

    // type SubscribeStream = ();
    //
    // async fn subscribe(&self, request: Request<Streaming<SubscribeRequest>>) -> Result<Response<Self::SubscribeStream>, Status> {
    //     todo!()
    //     // let r = request.into_inner();
    //     // match r.path {
    //     //     0 => Ok(Response::new(gnmi::CapabilityResponse {
    //     //         supported_models: vec![],
    //     //         supported_encodings: vec![],
    //     //         g_nmi_version: "0.8.0".to_string(),
    //     //         extension: vec![],
    //     //     })),
    //     //     1 => Ok(Response::new(voting::VotingResponse { confirmation: {
    //     //         format!("Confirmation that you for {}", r.url)
    //     //     }})),
    //     //     _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid vote provided"))
    //     // }
    //
    // }
}

impl GnmiService {
    fn stats(&self) {
        self.cnt.fetch_add(1, Ordering::Relaxed);
        self.cnt_period.fetch_add(1, Ordering::Relaxed);
        if self.cnt_period.load(Ordering::Relaxed) == 10000 {
            let duration = self.start.elapsed().as_secs();
            self.cnt_period.store(0, Ordering::Relaxed);
            info!("{}/{}/{:?}/", self.cnt.load(Ordering::Relaxed), self.cnt.load(Ordering::Relaxed) / (if duration > 0 { duration } else { 1 }), duration);
        }
    }
}
