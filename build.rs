// use std::env;
// use std::path::PathBuf;

fn main() {
	// let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	// tonic_build::configure()
	// 	.file_descriptor_set_path(out_dir.join("echo_descriptor.bin"))
	// 	.compile(&["proto/echo.proto"], &["proto"])
	// 	.unwrap();

	tonic_build::compile_protos("proto/echo.proto")
		.unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
