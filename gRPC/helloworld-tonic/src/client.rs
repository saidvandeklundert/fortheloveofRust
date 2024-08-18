use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloRequest, GoodbyeRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let hello_request = tonic::Request::new(HelloRequest {
        name: "Said".into(),
    });

    let hello_response = client.say_hello(hello_request).await?;

    println!("RESPONSE={:?}", hello_response);


    // goodbye
    let goodbye_request = tonic::Request::new(GoodbyeRequest {
        name: "Said".into(),
    });

    let goodbye_response = client.goodbye(goodbye_request).await?;

    println!("RESPONSE={:?}", goodbye_response);    
    Ok(())
}