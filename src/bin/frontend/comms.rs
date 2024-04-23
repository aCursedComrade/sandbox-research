use sandbox_research::{
    ipc_srv::{ipc_wire_client::IpcWireClient, EchoRequest},
    Status,
};
use std::sync::mpsc::Sender;
use tokio::runtime::Runtime;
use tonic::{transport::Channel, Request};

#[derive(Debug, Clone)]
/// Represents types of messages sent
/// through channels across threads.
pub enum QueueMsg {
    Spawn,
    Echo(String),
    Fail(Status),
}

async fn connect() -> Result<IpcWireClient<Channel>, ()> {
    match IpcWireClient::connect("http://[::1]:50055").await {
        Ok(client) => Ok(client),
        Err(error) => {
            tracing::error!("Cannot connect to server: {}", error);
            Err(())
        }
    }
}

/// `Echo` procedure (Testing only)
pub fn echo(sender: Sender<QueueMsg>, data: EchoRequest) {
    let request = Request::new(data);
    let mut msg = QueueMsg::Fail(Status::ConnectionFailed);

    tokio::spawn(async move {
        if let Ok(mut client) = connect().await {
            let response = client.echo(request).await;
            msg = match response {
                Ok(data) => QueueMsg::Echo(data.into_inner().message),
                Err(_) => QueueMsg::Fail(Status::ResponseError),
            }
        }

        if let Err(error) = sender.send(msg) {
            tracing::error!("Failed to send through channel: {}", error);
        }
    });
}

/// `Spawn` procedure
pub fn spawn(rt: &Runtime, sender: Sender<QueueMsg>, data: String) {
    todo!()
}
