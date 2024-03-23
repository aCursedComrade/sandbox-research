use crate::{
    sample_service::{exchange_client::ExchangeClient, EchoRequest},
    util::FailStatus,
};
// use std::sync::mpsc::Sender;
use tonic::{transport::Channel, Request};

#[derive(Debug, Clone)]
pub struct _Comms {}

async fn connect() -> Result<ExchangeClient<Channel>, FailStatus> {
    match ExchangeClient::connect("http://[::1]:50055").await {
        Ok(client) => Ok(client),
        Err(error) => {
            eprintln!("[!] Connect error: {}", error);
            Err(FailStatus::ConnectionFailed)
        }
    }
}

pub async fn echo(data: EchoRequest) -> Result<String, FailStatus> {
    let mut client = connect().await?;
    let request = Request::new(data);
    let response = client.echo(request).await;

    match response {
        Ok(data) => Ok(data.into_inner().message),
        Err(_) => Err(FailStatus::ResponseError),
    }
}
