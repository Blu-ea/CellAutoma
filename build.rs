use std::process::Command;

fn main() -> Result<(), String> {
    let status = Command::new("make")
        .status()
        .expect("failed to run make — is it installed and on PATH?");

    if !status.success() {
        return Result::Err(format!("make failed to compile shaders (exit code: {:?})", status.code().unwrap()) );
    }
    println!("cargo:rerun-if-changed=shader");
    Ok(())
}