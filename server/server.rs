use tonic::{transport::Server, Request, Response, Status};
use maily::maily_server::{Maily, MailyServer};
use maily::{SendRequest, SendReply};
use anyhow::Result;

pub mod smtp;

pub mod maily {
    tonic::include_proto!("maily");
}

pub struct Mail;

#[tonic::async_trait]
impl Maily for Mail {
    async fn send(
        &self,
        request: Request<SendRequest>,
    ) -> Result<Response<SendReply>, Status> {
        let body = request.into_inner();

        println!("{:?}", body);

        let reply = SendReply {
            message: "".to_string(),
            error: false,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let addr = format!("0.0.0.0:{}", dotenv::var("PORT")?).parse()?;

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(MailyServer::new(Mail))
        .serve(addr)
        .await?;

    Ok(())
}
