use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

pub struct LoggingLayer {
 
}

impl<S> Layer<S> for LoggingLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<S>) {
        let meta = event.metadata();

        let target = meta.target();

        let message = format!("target(): {}", target);

        println!("{}", message);
    }
}
