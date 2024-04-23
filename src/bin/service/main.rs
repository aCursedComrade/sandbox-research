use sandbox_research::ipc_srv::{
    ipc_wire_server::{IpcWire, IpcWireServer},
    EchoRequest, EchoResponse, SpawnRequest, SpawnResponse,
};
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct Service {}

#[tonic::async_trait]
impl IpcWire for Service {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let req = request.into_inner();
        println!("[*] Got a request: {:?}", &req);

        let message = EchoResponse {
            message: format!("Hello, you sent me: {}", req.payload),
        };

        Ok(Response::new(message))
    }

    async fn spawn(&self, request: Request<SpawnRequest>) -> Result<Response<SpawnResponse>, Status> {
        let req = request.into_inner();
        tracing::info!("Got a spawn request: {:?}", &req);

        let message = SpawnResponse { pid: 0 };

        Ok(Response::new(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50055".parse()?;
    let service = Service::default();

    Server::builder().add_service(IpcWireServer::new(service)).serve(addr).await?;

    Ok(())
}
