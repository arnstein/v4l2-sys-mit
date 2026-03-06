extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

	println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let incdir = format!("-I{}", env::var("VIDEODEV_PATH").unwrap_or_else(|_| "/usr/include".to_string()));

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(incdir);

    // V4L2_SYSROOT is used for cross-compilation (e.g. Yocto). Not needed for native builds.
    if let Ok(sysroot) = env::var("V4L2_SYSROOT") {
        builder = builder.clang_arg(format!("--sysroot={}", sysroot));
    }

    let bindings = builder
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("v4l2_bindings.rs"))
        .expect("Failed to write bindings");
}
