use std::net::SocketAddr;

use tonic::{transport::Server, Request, Response, Status};

use sample_service::exchange_server::{Exchange, ExchangeServer};
use sample_service::{EchoReply, EchoRequest};

pub(crate) mod sample_service {
    tonic::include_proto!("ipc_interface");
}

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl Exchange for Service {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoReply>, Status> {
        println!("[*] Got a request: {:?}", request);

        let response = sample_service::EchoReply {
            message: format!("Hello, you sent me: {}", request.into_inner().payload),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50055".parse()?;
    let service = Service::default();

    Server::builder()
        .add_service(ExchangeServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
