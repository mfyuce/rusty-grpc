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
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use log::{debug, error, info, LevelFilter, warn};
use redis::aio::ConnectionManager;
use redis::{PubSubCommands, ControlFlow, AsyncCommands, JsonCommands, RedisResult};
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};
use crate::gnmi::g_nmi_client::GNmiClient;
use std::collections::HashMap;
use std::thread::sleep;
// use tokio::time::sleep;
use tonic::codegen::ok;
extern crate redis;
use serde_json::json;
use tonic::transport::Channel;
use crate::redis::JsonAsyncCommands;

const DEFAULT_GNMI_SERVER_HOST_AND_PORT: &str = "[::]:8080";
const DEFAULT_REDIS_SERVER_HOST_AND_PORT: &str = "redis://:ulak@redis/";

const TEXT_GNMI_SERVER_HOST_AND_PORT: &'static str = "GNMI_SERVER_HOST_AND_PORT";
const TEXT_REDIS_SERVER_HOST_AND_PORT: &'static str = "REDIS_SERVER_HOST_AND_PORT";
/*
CONFIG SET notify-keyspace-events KEAm
*/
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let gnmi_url = match env::var_os(TEXT_GNMI_SERVER_HOST_AND_PORT) {
        Some(v) => v.into_string().unwrap().to_string(),
        None => DEFAULT_GNMI_SERVER_HOST_AND_PORT.to_string(),
    };
    let redis_url = match env::var_os(TEXT_REDIS_SERVER_HOST_AND_PORT) {
        Some(v) => v.into_string().unwrap().to_string(),
        None => DEFAULT_REDIS_SERVER_HOST_AND_PORT.to_string(),
    };
    let poolCM =  ConnectionManager::new(redis::Client::open(redis_url.clone()).unwrap()).await.unwrap();
    let pool =  redis::Client::open(redis_url.clone()).unwrap();

    let mut cnt = 0;
    let mut cntTmp = 0;
    // loop {
        let tmp_pool = pool.clone();
        // loop {
            let client_and_err = tmp_pool.get_connection();
            if client_and_err.is_ok()
            {
                // info!("connected to redis: {}",redis_url.clone());
                let mut con = client_and_err.unwrap();//client_and_err.unwrap();
                let mut pubsub = con.as_pubsub();

                let sub_and_result = pubsub.psubscribe("__keyspace@0__:maya_config_*:*");
                if sub_and_result.is_ok() {
                    loop {
                        // let _ = sleep(Duration::from_nanos(1));
                        let pubsub_msg_and_result = pubsub.get_message();
                        if pubsub_msg_and_result.is_ok() {
                            let msg = pubsub_msg_and_result.unwrap();
                            let payload_and_result = msg.get_payload();
                            if payload_and_result.is_ok() {
                                let payload: String = payload_and_result.unwrap();
                                let channel_name: String = msg.get_channel_name().parse().unwrap();
                                if payload.eq("json.set") && !channel_name.contains(":inProgress") && !channel_name.contains("done") {
                                    cnt += 1;
                                    trace!("channel '{}': {}", channel_name, payload);
                                    // info!("cnt '{}' ", cnt);
                                    let tmp_redis_url1 = redis_url.clone();
                                    let tmp_pool1 = poolCM.clone();
                                    tokio::spawn(async move {
                                        push_one_configuration(tmp_redis_url1, tmp_pool1.clone(), channel_name).await;
                                    });
                                    cntTmp+=1;
                                    // if cntTmp > 1000 {
                                    // let _ = thread::sleep(Duration::from_millis(10));
                                    //     cntTmp = 0;
                                    // }
                                }
                            } else {
                                error!("Unable get a payload from redis message: {}, retrying {}",redis_url, payload_and_result.err().unwrap());
                                sleep(Duration::from_secs(10));
                            }
                        } else {
                            error!("Unable get a message from redis: {}, retrying {}",redis_url, pubsub_msg_and_result.err().unwrap());
                            sleep(Duration::from_secs(10));
                        }
                    }
                } else {
                    error!("Unable subscribe to redis: {}, retrying {}",redis_url, sub_and_result.err().unwrap());
                    let _ = sleep(Duration::from_secs(10));
                }
            }
            else {
                error!("Unable to locate redis: {}, retrying {}",redis_url, client_and_err.err().unwrap());
                let _ = sleep(Duration::from_secs(10));
            }

            // let _ = sleep(Duration::from_nanos(1));
        // };
    // }

    Ok(())
}
async fn push_one_configuration(tmp_redis_url: String, mut pool_b: ConnectionManager, channel_name: String) {
    let redis_url = tmp_redis_url.clone();

    let config_name = str::replace(&*channel_name, "__keyspace@0__:", "");
    let config_name_tmp = config_name.clone();

    // let mut ip = str::replace(&*config_name_tmp, "maya_config_0:", "");
    // for i in 1..100 {
    //     ip = str::replace(&*config_name_tmp, &format!("maya_config_{}:", i), "");
    // }
    let res_and_result: RedisResult<u32> = pool_b.set_nx(config_name.clone() + ":inProgress", format!("{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis())).await;
    // let res_and_result: RedisResult<u32> = pool_b.json_set(config_name, "$.inProgress", &json!(true)).await;
    if res_and_result.is_ok()
    {
        let res = res_and_result.unwrap();
        if res == 1 {// key is set
            //later change to ip:<gnmi_port> above
            let gnmi_server = "http://[::1]:8080";
            let mut client:GNmiClient<Channel>;
            let mut cnt_try = 0;
            // let _ = tokio::time::sleep(Duration::from_nanos(1));
            loop {

                let client_and_error = GNmiClient::connect(gnmi_server).await;
                if client_and_error.is_ok() {
                    client = client_and_error.unwrap();
                    break;
                }else {
                    cnt_try +=1;
                    if cnt_try > 10 {
                        error!("gnmi connect error : {}", client_and_error.err().unwrap());
                        remove_inprogress(&mut pool_b, config_name.clone(), gnmi_server).await;
                        return ;//Err(());
                    }
                    let _ = sleep(Duration::from_secs(10));
                }
            }

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
            drop(client);

            if response_and_result.is_ok() {
                let res_and_result: RedisResult<u32> = pool_b.set_nx(config_name_tmp.clone() + ":done", "true").await;
                // let res_and_result: RedisResult<u32> = pool_b.json_set(config_name_tmp.clone(), "$.done", &json!(true)).await;
                if res_and_result.is_ok() {
                    let res = res_and_result.unwrap();
                    if res != 1 {
                        //if already done? there must be some problems in logic
                        assert!(1 == 1)
                    }
                    // info!("finished setting config for gnmi server {}",gnmi_server);
                }
            } else {
                // try to unset in progress
                error!("Unable to get response from gnmi server {}, {} ",gnmi_server, response_and_result.err().unwrap());

                remove_inprogress(&mut pool_b, config_name_tmp, gnmi_server).await;
            }

            // let _ = sleep(Duration::from_nanos(1));
            // }
        }
    } else {
        //go for the next
    }
}

async fn remove_inprogress(pool_b: &mut ConnectionManager, config_name_tmp: String, gnmi_server: &str) {
    let res_and_result: RedisResult<u32> = pool_b.del(config_name_tmp.clone() + ":inProgress").await;
    if !res_and_result.is_ok() {
        error!("Unable to delete key inProgress for  {}, {} ",gnmi_server, res_and_result.err().unwrap());
    }
}

