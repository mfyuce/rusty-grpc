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
const DEFAULT_REDIS_SERVER_HOST_AND_PORT: &str = "redis://:ulak@redis/";

const TEXT_GNMI_SERVER_HOST_AND_PORT: &'static str = "GNMI_SERVER_HOST_AND_PORT";
const TEXT_REDIS_SERVER_HOST_AND_PORT: &'static str = "REDIS_SERVER_HOST_AND_PORT";
/*
CONFIG SET notify-keyspace-events KEAm
*/
#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let gnmi_url =      match env::var_os(TEXT_GNMI_SERVER_HOST_AND_PORT) {
            Some(v) => v.into_string().unwrap().to_string(),
            None => DEFAULT_GNMI_SERVER_HOST_AND_PORT.to_string(),
        };
    let redis_url  =  match env::var_os(TEXT_REDIS_SERVER_HOST_AND_PORT) {
            Some(v) => v.into_string().unwrap().to_string(),
            None => DEFAULT_REDIS_SERVER_HOST_AND_PORT.to_string(),
        };
    let pool = r2d2::Pool::builder()
        .build(RedisConnectionManager::new(redis_url.clone()).unwrap())
        .unwrap();

    let tmp_redis_url = redis_url.clone();
    let tmp_pool = pool.clone();
    let thr = thread::spawn(move || {
        loop {
            let client_and_err = tmp_pool.get();
            if client_and_err.is_ok() {
                info!("connected to redis: {}",tmp_redis_url);
                let mut con = client_and_err.unwrap();
                let mut pubsub = con.as_pubsub();

                let sub_and_result = pubsub.psubscribe("__keyspace@0__:config:*");
                if sub_and_result.is_ok() {
                    loop {
                        let pubsub_msg_and_result = pubsub.get_message();
                        if pubsub_msg_and_result.is_ok() {
                            let msg = pubsub_msg_and_result.unwrap();
                            let payload_and_result  = msg.get_payload();
                            if payload_and_result.is_ok() {
                                let payload:String = payload_and_result.unwrap();
                                let channel_name: String = msg.get_channel_name().parse().unwrap();
                                if payload.eq("json.set") && !channel_name.contains(":inProgress") && !channel_name.contains("done") {
                                    info!("channel '{}': {}", channel_name, payload);
                                    push_one_configuration(&tmp_redis_url, tmp_pool.clone(), channel_name);
                                }
                            }else {
                                error!("Unable get a payload from redis message: {}, retrying {}",tmp_redis_url, payload_and_result.err().unwrap());
                                sleep(Duration::from_secs(10))
                            }
                        }else {
                            error!("Unable get a message from redis: {}, retrying {}",tmp_redis_url, pubsub_msg_and_result.err().unwrap());
                            sleep(Duration::from_secs(10))
                        }
                    }
                }else {
                    error!("Unable subscribe to redis: {}, retrying {}",tmp_redis_url, sub_and_result.err().unwrap());
                }
            }else {
                error!("Unable to locate redis: {}, retrying {}",tmp_redis_url, client_and_err.err().unwrap());
            }

            let _ = sleep(Duration::from_nanos(1));
        };
    });



    let gnmi_service = GnmiService::new(redis_url, pool.clone());
    Server::builder().add_service(GNmiServer::new(gnmi_service))
        .serve(gnmi_url.parse().unwrap())
        .await?;

    Ok(())
}
#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn push_one_configuration(tmp_redis_url: &String, pool_b: Pool<RedisConnectionManager>, channel_name: String) {
    let redis_url = tmp_redis_url.clone();
    tokio::spawn(async move {
        let _ = sleep(Duration::from_nanos(1));
        let client_and_err = pool_b.get();
        if client_and_err.is_ok() {
            let mut con_b = client_and_err.unwrap();
            let config_name = str::replace(&*channel_name, "__keyspace@0__:", "");
            let config_name_tmp = config_name .clone();
            let ip = str::replace(&*config_name_tmp, "config:", "");
            let res_and_result:RedisResult<u32> = con_b.set_nx(config_name + ":inProgress", "true");
            if res_and_result.is_ok()
            {
                let res = res_and_result.unwrap();
                if res == 1 {// key is set
                    //later change to ip:<gnmi_port> above
                    let gnmi_server = "http://[::1]:8080";
                    let mut client_and_error = GNmiClient::connect(gnmi_server).await;
                    if client_and_error.is_ok() {
                        let mut client = client_and_error.unwrap();
                        // loop {
                            let update = Update {
                                path: Option::from(Path {
                                    element: vec![],
                                    origin: "".to_string(),
                                    elem: vec![PathElem {
                                        name: "".to_string(),
                                        key: HashMap::from([("Norway".parse().unwrap(), "100".parse().unwrap()),
                                            ("Denmark".parse().unwrap(), "50".parse().unwrap()),
                                            ("Iceland".parse().unwrap(), "10".parse().unwrap())]),
                                    }],
                                    target: "".to_string(),
                                }),
                                value: None,
                                val: None,
                                duplicates: 0,
                            };
                            let request = tonic::Request::new(SetRequest {
                                prefix: None,
                                delete: vec![],
                                replace: vec![],
                                update: vec![update],
                                extension: vec![],
                            });
                            let response_and_result = client.set(request).await;

                            if response_and_result.is_ok() {
                                let res_and_result:RedisResult<u32> = con_b.set_nx(config_name_tmp.clone() + ":done", "true");
                                if res_and_result.is_ok() {
                                    let res = res_and_result.unwrap();
                                    if res != 1 {
                                        //if already done? there must be some problems in logic
                                        assert!(1==1)
                                    }
                                    info!("finished setting config for gnmi server {}",gnmi_server);
                                }
                            } else {
                                // try to unset in progress
                                error!("Unable to get response from gnmi server {}, {} ",gnmi_server, response_and_result.err().unwrap());

                                let res_and_result:RedisResult<String> = con_b.del(config_name_tmp.clone() + ":inProgress");
                                if !res_and_result.is_ok() {
                                    error!("Unable to delete key inProgress for  {}, {} ",gnmi_server, res_and_result.err().unwrap());
                                }
                            }

                            let _ = sleep(Duration::from_nanos(1));
                        // }
                    } else {
                        error!("version: {}", client_and_error.err().unwrap());
                    }
                }
            }else{
                //go for the next
            }
        } else {
            error!("Unable to locate redis: {}, retrying {}",redis_url, client_and_err.err().unwrap());
        }
         ""
    });
}

// #[derive(Debug, Default)]
pub struct GnmiService {
    pub pool: r2d2_redis::r2d2::Pool<RedisConnectionManager>,
    // pub pool: Pool<RedisConnection,RedisError>,
    pub cnt:AtomicU64,
    pub cnt_period: AtomicU64,
    pub  start: Instant,
}

#[tonic::async_trait]
impl GNmi for GnmiService {
    async fn capabilities(&self, request: Request<CapabilityRequest>) -> Result<Response<CapabilityResponse>, Status> {
        self.stats();
        let mut conn = self.pool.get().unwrap();

        // let option = request.remote_addr().unwrap();
        // let ip = option.ip().to_string();
        // let port = option.port();
        // let x: RedisResult<u16> = conn.get(option.ip().to_string());
        // if x.unwrap() != port {
        //     let _: () = conn.set(ip, port).expect("failed to execute SET for 'foo'");
        // }
        //     }
        // }
        // println!("Got: '{}' from redis", result.err().unwrap().to_string());
        // redis::cmd("SET").arg(&["key2", "bar"]).query_async(&mut con).await?;
        //
        // let result = redis::cmd("MGET")
        //     .arg(&["key1", "key2"])
        //     .query_async(&mut con)
        //     .await;
        // assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));

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

    fn new(redis_url: String, pool:r2d2_redis::r2d2::Pool<RedisConnectionManager>) -> Self {
        GnmiService {
            start: Instant::now(),
            cnt: AtomicU64::new(0),
            cnt_period: AtomicU64::new(0),
            pool
            //pool: RedisPool::new(RedisConnectionManager::new(Client::open(redis_url).unwrap(), true, None), 10000, )
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
        if self.cnt_period.load(Ordering::Relaxed) == 100000 {
            let duration = self.start.elapsed().as_secs();
            self.cnt_period.store(0, Ordering::Relaxed);
            info!("{}/{}/{:?}/", self.cnt.load(Ordering::Relaxed), self.cnt.load(Ordering::Relaxed) / (if duration > 0 { duration } else { 1 }), duration);
        }
    }
}
