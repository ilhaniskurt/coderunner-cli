use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Path to the binary
    #[arg(value_parser = validate_bin)]
    pub bin: PathBuf,
}

#[cfg(windows)]
fn is_bin(metadata: fs::Metadata) -> Result<bool, String> {
    // On Windows
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
    const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 0x2000;
    const FILE_ATTRIBUTE_DEVICE: u32 = 0x40;
    const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    const FILE_ATTRIBUTE_VIRTUAL: u32 = 0x10000;
    const FILE_ATTRIBUTE_INTEGRITY_STREAM: u32 = 0x8000;
    const FILE_ATTRIBUTE_NO_SCRUB_DATA: u32 = 0x20000;
    const FILE_ATTRIBUTE_RECALL_ON_OPEN: u32 = 0x40000;
    const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x400000;
    const FILE_ATTRIBUTE_RECALL_ON_CLOSE: u32 = 0x800000;

    let attributes = metadata.file_attributes();
    if attributes
        & (FILE_ATTRIBUTE_DIRECTORY
            | FILE_ATTRIBUTE_NOT_CONTENT_INDEXED
            | FILE_ATTRIBUTE_DEVICE
            | FILE_ATTRIBUTE_READONLY
            | FILE_ATTRIBUTE_REPARSE_POINT
            | FILE_ATTRIBUTE_VIRTUAL
            | FILE_ATTRIBUTE_INTEGRITY_STREAM
            | FILE_ATTRIBUTE_NO_SCRUB_DATA
            | FILE_ATTRIBUTE_RECALL_ON_OPEN
            | FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS
            | FILE_ATTRIBUTE_RECALL_ON_CLOSE)
        != 0
    {
        Err(format!(
            "File is not a executable (may not have the right permissions)!"
        ))
    } else {
        Ok(true)
    }
}

#[cfg(unix)]
fn is_bin(metadata: fs::Metadata) -> Result<bool, String> {
    // On Unix based OS
    use std::os::unix::fs::PermissionsExt;
    let mode: u32 = metadata.permissions().mode();
    if mode & 0o111 != 0 {
        Ok(true)
    } else {
        Err(format!(
            "File is not a executable (may not have the right permissions)!"
        ))
    }
}

fn validate_bin(s: &str) -> Result<PathBuf, String> {
    let path: PathBuf = PathBuf::from(s);
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Err(format!("Given path is a directory!"))
            } else {
                if let Err(error) = is_bin(metadata) {
                    Err(error)
                } else {
                    Ok(path)
                }
            }
        }
        Err(_) => Err(format!("Invalid path!")),
    }
}
