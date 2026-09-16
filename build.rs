use std::process::Command;

fn main() {
    let status = Command::new("make")
        .status()
        .expect("failed to run make — is it installed and on PATH?");

    if !status.success() {
        panic!("make failed to compile shaders (exit code: {:?})", status.code());
    }

    println!("cargo:rerun-if-changed=shader");
}