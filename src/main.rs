use tonic::{metadata::MetadataMap, transport::Server};

use opentelemetry::trace::TraceError;
use opentelemetry::sdk::trace;
use opentelemetry::sdk::trace::{IdGenerator, Sampler, Tracer};
use opentelemetry_otlp::Protocol;

use std::time::Duration;
use std::error::Error;

mod transport;

use transport::echo::echo_server::EchoServer;
use transport::EchoService;

fn tracing_init() -> Result<Tracer, TraceError> {
	let metadata = MetadataMap::new();

	let (tracer, _uninstall) = opentelemetry_otlp::new_pipeline()
		.with_protocol(Protocol::Grpc)
		.with_timeout(Duration::from_secs(3))
		.with_metadata(metadata)
		.with_trace_config(
			trace::config()
			.with_default_sampler(Sampler::AlwaysOn)
			.with_id_generator(IdGenerator::default())
			.with_max_events_per_span(64)
			.with_max_attributes_per_span(16)
			.with_max_events_per_span(16)
		)
		.install()?;

		Ok(tracer)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
	let _ = tracing_init()?;

	// let reflection_service = tonic_reflection::server::Builder::configure()
	//   .register_encoded_file_descriptor_set(echo::FILE_DESCRIPTOR_SET)
	//   .build()
	//   .unwrap();

	let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
	health_reporter
		.set_serving::<EchoServer<EchoService>>()
		.await;

		let addr = "[::1]:50051".parse()?;
		let echo = EchoService::default();

		println!("starting server on {}", addr);
		Server::builder()
			// .add_service(reflection_service)
			.add_service(health_service)
			.add_service(EchoServer::new(echo))
			.serve(addr)
			.await?;

		Ok(())
}
