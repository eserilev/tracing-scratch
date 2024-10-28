mod example_module;
use example_module::another_log;
use tracing::{info, info_span};

pub fn logging_from_another_crate() {
    let span = info_span!("", service = "test");
    let _guard = span.entered();
    info!("Testing a different crate");

    another_log();
}

