use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("cc")
        .args(&["src/ext.c", "-c", "-fPIC", "-o"])
        .arg(PathBuf::from(&out_dir).join("ext.o"))
        .status()
        .unwrap();
    Command::new("ar")
        .args(&["crus", "libext.a", "ext.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", &out_dir);
    println!("cargo:rerun-if-changed=src/ext.c");
}
