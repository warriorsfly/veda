#[macro_use]
extern crate lazy_static;
mod act;
mod config;
mod constants;
mod entity;
mod handler;
mod server;

use server::serv;

/// 消息gRPC
pub mod message {
    tonic::include_proto!("message");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
