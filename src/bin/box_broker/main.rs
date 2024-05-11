mod proc;
mod server;
mod state;

use sandbox_research::ipc_srv::ipc_wire_server::IpcWireServer;
use server::Service;
use std::net::SocketAddr;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = "[::1]:50055".parse()?;
    let service = Service::default();

    Server::builder().add_service(IpcWireServer::new(service)).serve(addr).await?;

    Ok(())
}
