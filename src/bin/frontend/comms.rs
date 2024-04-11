use crate::ipc_srv::{exchange_client::ExchangeClient, EchoRequest};
use sandbox_research::FailStatus;
use std::{process::Termination, sync::mpsc::Sender};
use tokio::runtime::Runtime;
use tonic::{transport::Channel, Request};

pub enum PipeMsg {
    Echo(String),
    Fail(FailStatus),
}

async fn connect() -> Result<ExchangeClient<Channel>, ()> {
    match ExchangeClient::connect("http://[::1]:50055").await {
        Ok(client) => Ok(client),
        Err(error) => {
            tracing::error!("[!] Cannot connect to server: {}", error);
            Err(())
        }
    }
}

pub fn echo(rt: &Runtime, sender: Sender<PipeMsg>, data: EchoRequest) {
    let request = Request::new(data);
    let mut msg = PipeMsg::Fail(FailStatus::ConnectionFailed);

    rt.spawn(async move {
        if let Ok(mut client) = connect().await {
            let response = client.echo(request).await;
            msg = match response {
                Ok(data) => PipeMsg::Echo(data.into_inner().message),
                Err(_) => PipeMsg::Fail(FailStatus::ResponseError),
            }
        }

        sender.send(msg).report();
    });
}
