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

        match smtp::send_mail(body.to, body.subject, body.content) {
            Ok(_) => {
                Ok(Response::new(SendReply {
                    message: "Email sent".to_string(),
                    error: false,
                }))
            },
            Err(e) => {
                eprintln!("{}", e);
                Ok(Response::new(SendReply {
                    message: e.to_string(),
                    error: true,
                }))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let addr = dotenv::var("ADDRESS")?.parse()?;

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(MailyServer::new(Mail))
        .serve(addr)
        .await?;

    Ok(())
}
