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
        if let Some(span_metadata) = _ctx.current_span().metadata() {
            // you can access the value of the fields by implementing tracing::field::Visit
            println!("span {:?}", span_metadata.fields());
        }

        let target = meta.target();

        let message = format!("target(): {}", target);

        println!("{}", message);
    }
}
