use sandbox_research::ipc_srv::{
    ipc_wire_server::{IpcWire, IpcWireServer},
    EchoRequest, EchoResponse, SpawnRequest, SpawnResponse, StopRequest, StopResponse,
};
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct Service {}

#[tonic::async_trait]
impl IpcWire for Service {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let req = request.into_inner();
        tracing::info!("Got an echo message: {:?}", &req.payload);

        let message = EchoResponse { message: req.payload };

        Ok(Response::new(message))
    }

    async fn spawn(&self, request: Request<SpawnRequest>) -> Result<Response<SpawnResponse>, Status> {
        let req = request.into_inner();

        // TODO process spawn logic goes here
        if let Some(profile) = req.profile {
            tracing::info!("Got a spawn request for: {:?}", &profile.name);
            let message = SpawnResponse {
                error: false,
                id: profile.id,
                pid: 69,
            };
            Ok(Response::new(message))
        } else {
            tracing::warn!("Invalid spawn request: {:?}", &req.profile);
            Err(Status::invalid_argument("Missing Profile data"))
        }
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        let req = request.into_inner();
        tracing::info!("Got a stop request: {:?}", &req.id);

        // TODO process stop logic goes here
        let message = StopResponse {
            error: false,
            id: req.id,
        };

        Ok(Response::new(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = "[::1]:50055".parse()?;
    let service = Service::default();

    Server::builder().add_service(IpcWireServer::new(service)).serve(addr).await?;

    Ok(())
}
