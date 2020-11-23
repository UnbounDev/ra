use tracingapp::tracing_app_client::TracingAppClient;
use tracingapp::EmptyRequest;

pub mod tracingapp {
    tonic::include_proto!("tracingapp");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TracingAppClient::connect("http://[::1]:8080").await?;

    let request = tonic::Request::new(EmptyRequest {});

    let response = client.get_version(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
