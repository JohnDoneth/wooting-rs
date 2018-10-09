extern crate bindgen;

extern crate cc;

use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./wooting-rgb-sdk/src/wooting-rgb-sdk.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let source_path = Path::new("./wooting-rgb-sdk/src");

    // This following section is Windows specific and needs to be 
    // refactored to be cross-platform
    cc::Build::new()
        .file("wooting-rgb-sdk/hidapi/windows/hid.c")
        .include("wooting-rgb-sdk/hidapi/hidapi/")
        .compile("hidapi");
    
    println!("cargo:rustc-link-lib=setupapi");

    cc::Build::new()
        .file("wooting-rgb-sdk/src/wooting-rgb-sdk.c")
        .file("wooting-rgb-sdk/src/wooting-usb.c")
        .include(source_path)
        .include("wooting-rgb-sdk/hidapi/hidapi/")
        .compile("wooting-rgb-sdk");
}