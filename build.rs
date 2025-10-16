use std::fs;
use std::path::{Path, PathBuf};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let static_src = PathBuf::from("static");
    let static_dst = out_dir.join("static");

    // Tell Cargo to re-run this build script if the static directory changes.
    println!("cargo:rerun-if-changed=static");

    if static_src.exists() {
        copy_dir_all(&static_src, &static_dst).expect("Failed to copy static files");
    }
}
