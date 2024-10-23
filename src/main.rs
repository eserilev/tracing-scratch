mod logging_layer;

use logging_layer::LoggingLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let custom_layer = LoggingLayer {};
    let _ = tracing_subscriber::fmt()
        .finish()
        .with(custom_layer)
        .try_init();

    example_crate::logging_from_another_crate()
}
