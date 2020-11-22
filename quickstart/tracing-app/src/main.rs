
extern crate ctrlc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};
use tracing::{info, span, trace, Level};
use tracing_subscriber;

// usage: `RUST_LOG=trace cargo run | jq .`
fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
      .json()
      .with_max_level(Level::TRACE)
      .init();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        let span = span!(Level::INFO, "exit_handler");
        let _enter = span.enter();
        r.store(false, Ordering::SeqCst);
        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);
        info!("Exit program");
    }).expect("Error setting Ctrl-C handler");

    let span = span!(Level::INFO, "root");
    span.in_scope(|| {
      let one_second = time::Duration::from_secs(1);
      while running.load(Ordering::SeqCst) {
          let span = span!(Level::INFO, "parent");
          span.in_scope(|| {
            thread::sleep(one_second);
            info!("{}", trivial());
          });
      }
    });
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