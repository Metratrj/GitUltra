use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=flatbuffers/git.fbs");

    let flatc_path = "bin/flatc"; // Path to the local flatc binary

    let status = Command::new(flatc_path)
        .args(&["--rust", "-o", "src", "flatbuffers/git.fbs"])
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => (),
        _ => panic!("Failed to generate Rust types: `flatc` not found or failed"),
    }
}
