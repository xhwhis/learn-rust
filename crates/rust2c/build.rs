extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("cc")
        .args(&["src/assign.c", "-c", "-fPIC", "-o"])
        .arg(PathBuf::from(&out_dir).join("assign.o"))
        .status()
        .unwrap();
    Command::new("ar")
        .args(&["crus", "libassign.a", "assign.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", &out_dir);
    println!("cargo:rustc-link-lib=static=assign");
    println!("cargo:rerun-if-changed=src/assign.h");
    let bindings = bindgen::Builder::default()
        .header("src/assign.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(PathBuf::from(&out_dir).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
