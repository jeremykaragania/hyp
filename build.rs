use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    const ASM: &[&str] = &["asm/head.S"];
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let cc = env::var("RUSTC_LINKER").unwrap();
    let linker_script = Path::new("linker.ld").canonicalize().unwrap();

    println!("cargo::rustc-link-arg=-T{}", linker_script.display());

    for file in ASM {
        println!("cargo::rerun-if-changed={file}");

        let filename = std::path::Path::new(file).file_stem().unwrap();
        let object = out.join(filename).with_extension("o");
        let status = Command::new(&cc)
            .args(["-c", file, "-o"])
            .arg(&object)
            .status()
            .unwrap();

        assert!(status.success());

        println!("cargo::rustc-link-arg={}", object.display());
    }

    println!("cargo::rerun-if-changed=linker.ld");
}
