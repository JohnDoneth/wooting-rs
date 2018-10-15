extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::path::Path;

use std::process::Command;

fn main() {

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let curr_dir = Path::new(&manifest_dir);
    Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .status()
        .unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./wooting-analog-sdk/src/wooting-analog-sdk.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


    let source_path = Path::new("./wooting-analog-sdk/src");

    // This following section is Windows specific and needs to be 
    // refactored to be cross-platform
    cc::Build::new()
        .file("wooting-analog-sdk/hidapi/windows/hid.c")
        .include("wooting-analog-sdk/hidapi/hidapi/")
        .compile("hidapi");
    
    println!("cargo:rustc-link-lib=setupapi");
    //n

    cc::Build::new()
        .file("wooting-analog-sdk/src/wooting-analog-sdk.c")
        .include(source_path)
        .include("wooting-analog-sdk/hidapi/hidapi/")
        .compile("wooting-analog-sdk");

    
}