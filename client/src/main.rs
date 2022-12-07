mod gnmi;
mod gnmi_ext;

use std::collections::LinkedList;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::gnmi::{CapabilityRequest, CapabilityResponse, GetRequest, GetResponse, SetRequest, SetResponse, SubscribeRequest};
use crate::gnmi::g_nmi_client::{GNmiClient};

#[tokio::main(flavor = "current_thread")] //, worker_threads = 2
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GNmiClient::connect("http://[::1]:8080").await?;
    let mut cnt = 0;
    let mut cntTmp = 0;
    let start = Instant::now();
    // let mut list: Vec<thread::JoinHandle<_>> = Vec::new();
    //
    // for i in 1..10 {
    //     let mut handle = thread::spawn(   || {

            loop {
                // println!("\nPlease vote for a particular url");
                // let mut u = String::new();
                // let mut vote: String = String::new();
                // println!("Please provide a url: ");
                // stdin().read_line(&mut u).unwrap();
                // let u = u.trim();
                // println!("Please vote (d)own or (u)p: ");
                // stdin().read_line(&mut vote).unwrap();
                // let v = match vote.trim().to_lowercase().chars().next().unwrap() {
                //     'u' => 0,
                //     'd' => 1,
                //     _ => break,
                // };
                let request = tonic::Request::new(CapabilityRequest {
                    extension: vec![]
                });
                let response = client.capabilities(request).await?;

                cnt=cnt+1;
                cntTmp=cntTmp+1;
                //println!("Got: '{}' from service", response.into_inner().g_nmi_version);
                if cntTmp == 1000 {
                    let duration = start.elapsed().as_secs();
                    cntTmp=0;
                    println!("{}/{}/{:?}/", cnt,cnt/(if duration>0 {duration} else { 1 }),duration);
                }
            }
    //     });
    //     list.push( handle);
    // }

    // while list.len() > 0 {
    //     let cur_thread = list.remove(0); // moves it into cur_thread
    //     cur_thread.join().unwrap();
    // }
    //
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    Ok(())
}
