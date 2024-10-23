mod example_module;
use example_module::another_log;
use tracing::info;

pub fn logging_from_another_crate() {
    info!("Testing a different crate");

    another_log();
}

