mod ipc_srv {
    tonic::include_proto!("ipc_interface");
}

use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

use ipc_srv::exchange_server::{Exchange, ExchangeServer};
use ipc_srv::{EchoReply, EchoRequest};

#[derive(Default)]
pub struct Service {}

#[tonic::async_trait]
impl Exchange for Service {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoReply>, Status> {
        let message = request.into_inner();
        println!("[*] Got a request: {:?}", &message);

        let response = ipc_srv::EchoReply {
            message: format!("Hello, you sent me: {}", message.payload),
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
