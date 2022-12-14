// #![feature(async_closure)]
#[macro_use] extern crate log;
extern crate simplelog;
mod gnmi;
mod gnmi_ext;

use std::collections::LinkedList;
use std::fs::File;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{env, thread};
use std::time::{Duration, Instant};
use log::{debug, error, info, LevelFilter, warn};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, JsonCommands};
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};
use tokio::task::JoinHandle;
use tokio::time::sleep;
extern crate redis;
use serde_json::json;
use crate::redis::JsonAsyncCommands;

use crate::gnmi::{CapabilityRequest, CapabilityResponse, GetRequest, GetResponse, SetRequest, SetResponse, SubscribeRequest};
use crate::gnmi::g_nmi_client::{GNmiClient};

const DEFAULT_GNMI_SERVER_HOST_AND_PORT: &str = "[::]:8080";
const DEFAULT_REDIS_SERVER_HOST_AND_PORT: &str = "redis://:ulak@redis/";

const TEXT_GNMI_SERVER_HOST_AND_PORT: &'static str = "GNMI_SERVER_HOST_AND_PORT";
const TEXT_REDIS_SERVER_HOST_AND_PORT: &'static str = "REDIS_SERVER_HOST_AND_PORT";

#[tokio::main(flavor = "multi_thread", worker_threads = 3)] //
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    // let cnt = AtomicU64::new(0);
    // let cnt_period = AtomicU64::new(0);
    // let start = Instant::now();
    let mut list: Vec<JoinHandle<()>> = Vec::new();

    let gnmi_url =      match env::var_os(TEXT_GNMI_SERVER_HOST_AND_PORT) {
            Some(v) => v.into_string().unwrap().to_string(),
            None => DEFAULT_GNMI_SERVER_HOST_AND_PORT.to_string(),
        };
    let redis_url  =  match env::var_os(TEXT_REDIS_SERVER_HOST_AND_PORT) {
            Some(v) => v.into_string().unwrap().to_string(),
            None => DEFAULT_REDIS_SERVER_HOST_AND_PORT.to_string(),
        };
    let pool =  ConnectionManager::new(redis::Client::open(redis_url).unwrap()).await.unwrap();
    // for i in 1..2 {
    //     let tmp_pool = pool.clone();
    //     let handle = tokio::spawn (  async move {
    //         bulk_try_with_gnmi( ).await
    //     });
    //     list.push( handle);
    // }

    let mut cur_bucket_number_tmp = 0;
    let mut cur_bucket_number = 0;
    let mut err_cnt = 0;
    let mut total_cnt = 0;
    for n1 in 1..255 {
        let tmp_pool = pool.clone();
        let handle = tokio::spawn (  async move {
            bulk_try_with_redis(n1,cur_bucket_number,tmp_pool.clone()).await
        });
        cur_bucket_number_tmp+=254;
        if cur_bucket_number_tmp >= 9000 {
            // cur_bucket_number += 1;
            cur_bucket_number_tmp = 0;
        }
        list.push( handle);
    }

    info!("total_cnt {}",total_cnt );
    if err_cnt > 0 {
        info!("err_cnt {}",err_cnt );
    }
    while list.len() > 0 {
        let mut cur_thread = list.remove(0); // moves it into cur_thread
        cur_thread.await.unwrap();
    }

    for i in 1..5 {
        info!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

async fn bulk_try_with_redis(n1: i32, cur_bucket_number: i32, mut pool: ConnectionManager) {
    for n2 in 1..254{
        let config_name =format!("maya_config_{}:192.168.{}.{}",cur_bucket_number, n1 ,n2);
        // let res:String = pool.set(config_name.clone(),"").await.unwrap();
        let res:String = pool.json_set(config_name,"$",&json!({"cfg": {}})).await.unwrap();
        // let res:u32 = pool.del(config_name ).await.unwrap();
         if res != "OK" {
            info!("{}",res );
        }
    }

}

async fn bulk_try_with_gnmi() {
    let mut client_and_error = GNmiClient::connect("http://[::1]:8080").await;
    if client_and_error.is_ok() {
        let mut client = client_and_error.unwrap();
        loop {
            let request = tonic::Request::new(CapabilityRequest {
                extension: vec![]
            });
            let response = client.capabilities(request).await;

            // debug!("version: {}",response.unwrap().into_inner().g_nmi_version);
            // // stats(cnt, cnt_period, start);
            // cnt.fetch_add(1, Ordering::Relaxed);
            // cnt_period.fetch_add(1, Ordering::Relaxed);
            // if cnt_period.load(Ordering::Relaxed) == 100000 {
            //     let duration = start.elapsed().as_secs();
            //     cnt_period.store(0, Ordering::Relaxed);
            //     println!("{}/{}/{:?}/", cnt.load(Ordering::Relaxed), cnt.load(Ordering::Relaxed) / (if duration > 0 { duration } else { 1 }), duration);
            // }
            // let _ = sleep(Duration::from_nanos(1));
        }
    } else {
        error!("version: {}", client_and_error.err().unwrap());
    }
}
