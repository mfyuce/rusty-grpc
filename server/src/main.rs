#![feature(local_key_cell_methods)]
mod gnmi;
mod gnmi_ext;

use std::ops::{Add, Deref};
use tonic::{transport::Server, Request, Response, Status};
use crate::gnmi::{CapabilityRequest, CapabilityResponse, GetRequest, GetResponse, SetRequest, SetResponse};
use crate::gnmi::g_nmi_server::{GNmi, GNmiServer};
// use std::thread;
use std::string::ToString;
use redis::{Client,  RedisError, RedisResult};
use std::error::Error;
use redis_async_pool::{RedisConnection, RedisConnectionManager, RedisPool};
use redis::AsyncCommands;
use std::borrow::Borrow;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::time::Instant;
use redis_async_pool::deadpool::managed::Pool;


// const GNMI_SERVER_HOST_AND_PORT: String = "[::]:8080".to_string();
// const REDIS_SERVER_HOST_AND_PORT: String = "redis://redis/".to_string();

// static mut CURRENT_GNMI_SERVER_HOST_AND_PORT: &str = &*GNMI_SERVER_HOST_AND_PORT;
// static mut CURRENT_REDIS_SERVER_HOST_AND_PORT: &str = &*REDIS_SERVER_HOST_AND_PORT;

// static client: Client = redis::Client::open("redis://127.0.0.1/").unwrap();
extern crate redis;
// fn do_something() -> redis::RedisResult<()> {
//     let client = redis::Client::open(CURRENT_GNMI_SERVER_HOST_AND_PORT)?;
//     let mut con = client.get_connection()?;
//
//     /* do something here */
//     Ok(())
// }
// fn connect() -> RedisResult<Connection> {
//     //format - host:port
//     let redis_host_name =  "redis"  ;
//
//     let redis_password = env::var("REDIS_PASSWORD").unwrap_or("ulak".to_string());
//     //if Redis server needs secure connection
//     let uri_scheme = match env::var("REDIS_IS_TLS") {
//         Ok(_) => "rediss",
//         Err(_) => "redis",
//     };
//     let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
//     let result = retry_with_index(Fixed::from_millis(100), |current_try| {
//         let ret = redis::Client::open(redis_conn_url.borrow());
//         if ret.is_err() {
//             if current_try < 10 {
//                 return OperationResult::Retry("Invalid connection URL");
//             }
//             return OperationResult::Err("Invalid connection URL");
//         }
//         let ret1 = ret.unwrap().get_connection();
//
//         if ret1.is_err() {
//             if current_try < 10 {
//                 return OperationResult::Retry("failed to connect to Redis");
//             }
//             return OperationResult::Err("failed to connect to Redis");
//         }
//         OperationResult::Ok(ret1)
//     });
//
//     return result.unwrap();
// }



    #[tokio::main(flavor = "multi_thread", worker_threads = 3)]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {

        //
        // let tmp:dyn Borrow<String> =      match env::var_os("GNMI_SERVER_HOST_AND_PORT") {
        //         Some(v) => v.into_string().unwrap().to_string(),
        //         None => GNMI_SERVER_HOST_AND_PORT.to_string(),
        //     }.borrow() ;
        // unsafe { CURRENT_GNMI_SERVER_HOST_AND_PORT = tmp.borrow(); }
        // let tmp1:dyn Borrow<Borrowed> =  match env::var_os("REDIS_SERVER_HOST_AND_PORT") {
        //         Some(v) => v.into_string().unwrap().to_string(),
        //         None => REDIS_SERVER_HOST_AND_PORT.to_string(),
        //     }.borrow() ;
        // unsafe { CURRENT_REDIS_SERVER_HOST_AND_PORT = tmp1.borrow(); }
        // let address = host_port.parse().unwrap();
        // let voting_service = VotingService::default();
        let gnmi_service = GnmiService::new();
        // client: Client = redis::Client::open("redis://127.0.0.1/").unwrap();
        Server::builder().add_service(GNmiServer::new(gnmi_service))
            .serve("[::]:8080".parse().unwrap())
            .await?;
        Ok(())
    }

// #[derive(Debug, Default)]
pub struct GnmiService {
    pub pool: Pool<RedisConnection,RedisError>,
    pub cnt:AtomicU64,
    pub cntPeriod: AtomicU64,
    pub  start: Instant,
}

#[tonic::async_trait]
impl GNmi for GnmiService {

    fn new() -> Self {
        GnmiService {start: Instant::now(), cnt:AtomicU64::new(0), cntPeriod:AtomicU64::new(0), pool: RedisPool::new(RedisConnectionManager::new(Client::open("redis://:ulak@localhost:6379").unwrap(), true, None), 5,)}
    }
    async fn capabilities(&self, request: Request<CapabilityRequest>) -> Result<Response<CapabilityResponse>, Status> {
        // todo!()
        self.cnt.fetch_add(1, Ordering::Relaxed);
        self.cntPeriod.fetch_add(1, Ordering::Relaxed);
        if self.cntPeriod.load(Ordering::Relaxed) == 1000 {
            let duration = self.start.elapsed().as_secs();
            self.cntPeriod.store(0, Ordering::Relaxed);
            println!("{}/{}/{:?}/", self.cnt.load(Ordering::Relaxed), self.cnt.load(Ordering::Relaxed) /(if duration>0 {duration} else { 1 }), duration);
        }
        // (&self.cntTmp+1).borrow();
        // if self.start == None {
        //     (&self.start).borrow(Instant::now());
        // }
        // //println!("Got: '{}' from service", response.into_inner().g_nmi_version);
        // if self.cntTmp == 1000 {
        //     let duration = self.start.elapsed().as_secs();
        //     (&self.cntTmp).borrow(0);
        //     println!("{}/{}/{:?}/", self.cnt,self.cnt/(if duration>0 {duration} else { 1 }),duration);
        // }

        // get a connection with the get() async method and use it as regular redis connection
        // let pool1: Pool<RedisConnection, RedisError> = RedisPool::new(
        //     RedisConnectionManager::new(Client::open("redis://localhost:6379").unwrap(), true, None),
        //     5,
        // );
        // let pool: Pool<RedisConnection, RedisError> = RedisPool::new(
        //     RedisConnectionManager::new(Client::open("redis://:ulak@localhost:6379").expect(""), true, None),
        //     5,
        // );
        let mut conn = self.pool.get().await.unwrap();
        // con.set(b"key", b"value").await?;
        // let value: Vec<u8> = con.get(b"key").await?;
        // assert_eq!(value, b"value");
        // let connTmp = connect();
        // if connTmp.is_ok() {
        // let connTmp1 = connTmp.unwrap();
        // if connTmp1.is_ok() {
        // let mut conn = connTmp.unwrap();

        let option = request.remote_addr().unwrap();
        let ip = option.ip().to_string();
        let port = option.port();
        let mut x: RedisResult<u16> = conn.get(option.ip().to_string()).await;
        if x.unwrap() != port {
            let _: () = conn.set(ip, port).await.expect("failed to execute SET for 'foo'");
        }
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

        let r = request.into_inner();
        return Ok(Response::new(gnmi::CapabilityResponse {
            supported_models: vec![],
            supported_encodings: vec![],
            g_nmi_version: "0.8.0".to_string(),
            extension: vec![],
        }));
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        todo!()
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        todo!()
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
    //     //         format!("Confirmation that you downvoted for {}", r.url)
    //     //     }})),
    //     //     _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid vote provided"))
    //     // }
    //
    // }


    // #[tonic::async_trait]
    // impl Voting for VotingService {
    //     async fn vote(&self, request: Request<VotingRequest>) -> Result<Response<VotingResponse>, Status> {
    //         let r = request.into_inner();
    //         match r.vote {
    //             0 => Ok(Response::new(voting::VotingResponse { confirmation: {
    //                 format!("Happy to confirm that you upvoted for {}", r.url)
    //             }})),
    //             1 => Ok(Response::new(voting::VotingResponse { confirmation: {
    //                 format!("Confirmation that you downvoted for {}", r.url)
    //             }})),
    //             _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid vote provided"))
    //         }
    //     }
    // }
}
