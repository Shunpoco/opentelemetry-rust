use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::logs::{BatchConfigBuilder, BatchLogProcessor, SdkLoggerProvider};
use opentelemetry_sdk::Resource;
use std::thread::sleep;
use std::time::Duration;
use tracing::error;
use tracing_subscriber::prelude::*;

fn main() {
    let exporter = opentelemetry_stdout::LogExporter::default();
    let processor = BatchLogProcessor::builder(exporter)
        .with_batch_config(
            BatchConfigBuilder::default()
                .with_max_queue_size(2048)
                .with_max_export_batch_size(512)
                .with_scheduled_delay(Duration::from_secs(5))
                .build(),
        )
        .build();

    let provider = SdkLoggerProvider::builder()
        .with_resource(Resource::builder().with_service_name("log-test").build())
        .with_log_processor(processor)
        .build();

    let layer = layer::OpenTelemetryTracingBridge::new(&provider);
    tracing_subscriber::registry().with(layer).init();

    for i in 0..1 {
        let message = format!("message{i}");
        error!(name: "my-event-name", target: "my-system", event_id=20, user_name="otel", user_email="otel@opentelemetry.io", message=message);
    }

    sleep(Duration::from_secs(10));

    let _ = provider.shutdown();
}
