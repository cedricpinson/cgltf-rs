use std::env;
use std::path::PathBuf;

use bindgen;
use cc;

fn main() {
    println!("cargo:rerun-if-changed=src/cgltf.h");
    println!("cargo:rerun-if-changed=src/cgltf_lib.c");

    cc::Build::new().file("src/cgltf_lib.c").compile("cgltf");

    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=cgltf");
    //    println!("cargo:rustc-link-lib=gltf_lib");

    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
