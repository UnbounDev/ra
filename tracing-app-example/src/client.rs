
use tracingapp::tracing_app_client::TracingAppClient;
use tracingapp::EmptyRequest;

mod constants;

pub mod tracingapp {
    tonic::include_proto!("tracingapp");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = format!("http://{}", constants::SERVER_ADDR);
    println!("connecting to {}", addr);
    let mut client = TracingAppClient::connect(addr).await?;

    let request = tonic::Request::new(EmptyRequest {});
    let response = client.get_version(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
