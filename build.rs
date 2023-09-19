extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=wolftpm");

    println!("cargo:rerun-if-changed=wrapper.h");

    let builder = bindgen::Builder::default()
        .size_t_is_usize(false)
        .header("wrapper.h")
        .blocklist_item("IPPORT_RESERVED"); // probably bug in bindgen. Produces duplicate definition

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("tpm2.rs"))
        .expect("Couldn't write bindings!");
}
