#[cfg(debug_assertions)]
use sandbox_research::ipc_srv::EchoRequest;
use sandbox_research::{
    ipc_srv::{ipc_wire_client::IpcWireClient, SpawnRequest, StopRequest},
    Profile, Status,
};
use std::sync::mpsc::Sender;
use tonic::{transport::Channel, Request};

#[derive(Debug, Clone)]
/// Represents types of messages sent
/// through channels across threads.
pub enum QueueMsg {
    Spawn {
        error: bool,
        id: u32,
        pid: u32,
    },
    Stop {
        error: bool,
        id: u32,
    },
    #[cfg(debug_assertions)]
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

#[cfg(debug_assertions)]
/// `Echo` procedure (Testing only)
pub fn echo<S: Into<String>>(sender: Sender<QueueMsg>, payload: S) {
    let request = Request::new(EchoRequest { payload: payload.into() });
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
pub fn spawn(sender: Sender<QueueMsg>, profile: Profile) {
    let request = Request::new(SpawnRequest {
        profile: Some(profile.into()),
    });
    let mut msg = QueueMsg::Fail(Status::ConnectionFailed);

    tokio::spawn(async move {
        if let Ok(mut client) = connect().await {
            let response = client.spawn(request).await;
            msg = match response {
                Ok(data) => {
                    let res = data.into_inner();
                    QueueMsg::Spawn {
                        error: res.error,
                        id: res.id,
                        pid: res.pid,
                    }
                }
                Err(_) => QueueMsg::Fail(Status::ResponseError),
            }
        }

        if let Err(error) = sender.send(msg) {
            tracing::error!("Failed to send through channel: {}", error);
        }
    });
}

/// `Stop` procedure
pub fn stop(sender: Sender<QueueMsg>, id: u32) {
    let request = Request::new(StopRequest { id });
    let mut msg = QueueMsg::Fail(Status::ConnectionFailed);

    tokio::spawn(async move {
        if let Ok(mut client) = connect().await {
            let response = client.stop(request).await;
            msg = match response {
                Ok(data) => {
                    let res = data.into_inner();
                    QueueMsg::Stop {
                        error: res.error,
                        id: res.id,
                    }
                }
                Err(_) => QueueMsg::Fail(Status::ResponseError),
            }
        }

        if let Err(error) = sender.send(msg) {
            tracing::error!("Failed to send through channel: {}", error);
        }
    });
}
