extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

	println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let sysroot = format!("--sysroot={}", env::var("LIBCLANG_PATH").unwrap());
    let incdir = format!("-I{}", env::var("VIDEODEV_PATH").unwrap());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
		.clang_arg(sysroot)
		.clang_arg(incdir)
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("v4l2_bindings.rs"))
        .expect("Failed to write bindings");
}
