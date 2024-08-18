/// gRPC server example
/// 
/// To run it use: cargo run --bin helloworld-server
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest, GoodbyeRequest, GoodbyeReply};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    // implementation of SayHello rpc:
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
    // implementation of SayHello rpc:
    async fn goodbye(
        &self,
        request: Request<GoodbyeRequest>,
    ) -> Result<Response<GoodbyeReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = GoodbyeReply {
            message: format!("Goodbye {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}