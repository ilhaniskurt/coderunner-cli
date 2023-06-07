use std::path::PathBuf;
use std::process::{Command, Stdio};

//
// Public Library Functions
//

// Run binary with no inputs
//
pub fn run_static(path: PathBuf) {
    // Check if path is absolute
    let full_path: PathBuf = get_full_path(path);

    // Binary process
    let output = Command::new(full_path)
        .output()
        .expect("failed to execute binary");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

pub fn run(path: PathBuf) {
    // Check if path is absolute
    let full_path: PathBuf = get_full_path(path);

    // Binary process
    let _process = Command::new(full_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute binary!");
}

//
// Private Library Functions
//

// For checking and resolving relative path
//
fn get_full_path(path: PathBuf) -> PathBuf {
    // If path is already absolute
    if path.is_absolute() {
        path
    } else {
        log::warn!("Relative path detected! Should use absolute path instead.");
        match path.canonicalize() {
            Ok(p) => p,
            Err(_) => panic!("Program paniced trying to resolve relative path!"),
        }
    }
}
