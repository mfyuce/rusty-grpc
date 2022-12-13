#![feature(async_closure)]
mod gnmi;
mod gnmi_ext;

use std::collections::LinkedList;
use std::fs::File;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use log::{debug, error, info, LevelFilter, warn};
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};
use tokio::task::JoinHandle;
use tokio::time::sleep;

use crate::gnmi::{CapabilityRequest, CapabilityResponse, GetRequest, GetResponse, SetRequest, SetResponse, SubscribeRequest};
use crate::gnmi::g_nmi_client::{GNmiClient};
//
// fn stats(cnt:AtomicU64,cnt_period:AtomicU64,start:Instant) {
//     cnt.fetch_add(1, Ordering::Relaxed);
//     cnt_period.fetch_add(1, Ordering::Relaxed);
//     if cnt_period.load(Ordering::Relaxed) == 100000 {
//         let duration = start.elapsed().as_secs();
//         cnt_period.store(0, Ordering::Relaxed);
//         println!("{}/{}/{:?}/", cnt.load(Ordering::Relaxed), cnt.load(Ordering::Relaxed) / (if duration > 0 { duration } else { 1 }), duration);
//     }
// }
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

    for i in 1..10 {
        let mut handle = tokio::spawn (  async move {

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
                    let _ = sleep(Duration::from_nanos(1));
                }
            }else {
                error!("version: {}", client_and_error.err().unwrap());
            }
        });
        list.push( handle);
    }

    while list.len() > 0 {
        let mut cur_thread = list.remove(0); // moves it into cur_thread
        cur_thread.await.unwrap();
    }

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
