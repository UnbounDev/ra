extern crate ctrlc;

use std::time;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{debug, info, span, trace, Level};
use tracing_subscriber;

use tracingapp::tracing_app_server::{TracingApp, TracingAppServer};
use tracingapp::{EmptyRequest, VersionResponse};

pub mod tracingapp {
    tonic::include_proto!("tracingapp");
}

#[derive(Debug, Default)]
pub struct App {}

#[tonic::async_trait]
impl TracingApp for App {
    #[tracing::instrument]
    async fn get_version(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<VersionResponse>, Status> {
        info!("Got a request: {:?}", request);

        let _now = trivial();

        let reply = VersionResponse {
            version: String::from(env!("CARGO_PKG_VERSION"))
        };

        Ok(Response::new(reply))
    }
}

// usage: `RUST_LOG=trace cargo run --bin tracingexampleapp-server | jq .`
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8080".parse()?;
    let app = App::default();

    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        //        .json()
        //        .with_max_level(Level::TRACE)
        .init();

    ctrlc::set_handler(move || {
        info!("shutting down");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    info!("listening on {}", addr);
    Server::builder()
        .add_service(TracingAppServer::new(app))
        .serve(addr)
        .await?;

    Ok(())
}

#[tracing::instrument]
fn trivial() -> String {
    let now = format!("{:?}", time::SystemTime::now());
    let span = span!(Level::TRACE, "child", now = now.as_str());
    // _enter acts as a guard, when dropped (out of scope) the span will close
    let _enter = span.enter();
    trace!("got systemtime");
    now
}
