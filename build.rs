use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell Cargo to re-run this build script if anything in the gowall_src directory changes
    println!("cargo:rerun-if-changed=gowall_src");

    // Retrieve the OUT_DIR provided by Cargo, e.g., target/debug/build/wallmod-HASH/out
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut target_dir = PathBuf::from(out_dir);

    // Pop 'out', 'wallmod-HASH', and 'build' to reach 'target/debug' or 'target/release'
    target_dir.pop();
    target_dir.pop();
    target_dir.pop();

    let gowall_binary = target_dir.join("gowall");

    // We print a warning just so it shows up in cargo's output for debugging
    println!(
        "cargo:warning=Building Go sidecar binary: {:?}",
        gowall_binary
    );

    // Execute `go build` inside the gowall_src directory, placing the binary in target_dir
    let status = Command::new("go")
        .current_dir("gowall_src")
        .arg("build")
        .arg("-o")
        .arg(&gowall_binary)
        .arg("main.go")
        .status()
        .expect("Failed to execute `go build`. Is Go installed?");

    if !status.success() {
        panic!("Failed to build the gowall Go sidecar binary!");
    }
}
