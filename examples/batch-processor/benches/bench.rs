use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::logs::{BatchConfigBuilder, BatchLogProcessor, SdkLoggerProvider};
use opentelemetry_sdk::Resource;
use std::time::Duration;
use tracing::warn;
use tracing_subscriber::prelude::*;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let exporter = opentelemetry_stdout::LogExporter::default();
    let processor = BatchLogProcessor::builder(exporter)
        .with_batch_config(
            BatchConfigBuilder::default()
                .with_max_queue_size(1024)
                .with_max_export_batch_size(512)
                .with_scheduled_delay(Duration::from_secs(3))
                .build(),
        )
        .build();

    let provider = SdkLoggerProvider::builder()
        .with_resource(Resource::builder().with_service_name("log-test").build())
        .with_log_processor(processor)
        .build();

    let layer = layer::OpenTelemetryTracingBridge::new(&provider);
    tracing_subscriber::registry().with(layer).init();

    c.bench_function("ifb 20", |b| {
        b.iter(|| {
            for i in 0..100 {
                let message = format!("message{i}");
                warn!(
                    name: "my-event-name",
                    target: "my-system",
                    event_id=20,
                    user_name="otel",
                    user_email="otel@opentelemetry.io",
                    message=message,
                );
            }
        })
    });

    let _ = provider.shutdown();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
