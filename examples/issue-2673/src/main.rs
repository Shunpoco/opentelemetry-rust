use opentelemetry_appender_tracing::layer;
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::Resource;
use std::{error::Error, sync::OnceLock};
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn init_log_provider<X, T>(service_name: X, endpoint: T) -> SdkLoggerProvider
where
    X: ToString + Into<opentelemetry::Value>,
    T: Into<String>,
{
    let client = reqwest::Client::new();
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_endpoint(endpoint)
        .with_protocol(opentelemetry_otlp::Protocol::HttpJson)
        .with_http_client(client)
        .build()
        .unwrap();

    let provider = SdkLoggerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(Resource::builder().with_service_name(service_name).build())
        .build();

    return provider
}

fn main() {
    println!("Hello, world!");
}
