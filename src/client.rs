use std::time::{SystemTime, UNIX_EPOCH};
use maily::maily_client::MailyClient;
use maily::SendRequest;
use anyhow::Result;

pub mod maily {
    tonic::include_proto!("maily");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = MailyClient::connect("http://0.0.0.0:50051").await?;

    let request = tonic::Request::new(SendRequest {
        to: "hinomegnome@gmail.com".to_string(),
        subject: "Hello".to_string(),
        content: "Hello".to_string()
    });

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let response = client.send(request).await?;
    println!("Response with {:?} in {}ms", response.into_inner().message, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()-start);

    Ok(())
}