use std::env;
use std::path::PathBuf;

fn main() {
	let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let out_path = PathBuf::from("target/include/rust.h");

	cbindgen::Builder::new()
		.with_crate(crate_dir)
		.with_language(cbindgen::Language::C)
		.generate()
		.expect("Unable to generate bindings")
		.write_to_file(out_path);
}
