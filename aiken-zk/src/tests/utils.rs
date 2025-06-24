use std::{env, fs};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use tempfile::{tempdir, TempDir};

fn copy_dir_contents(src: &Path, dst: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_contents(&entry_path, &dst_path)?; // recursive copy
        } else {
            fs::copy(&entry_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn create_sandbox_and_set_as_current_directory() -> TempDir {
    let sandbox_path = &sandbox_path();
    let source = Path::new(sandbox_path);
    let temp_dir = tempdir().expect("Could not create temp dir");
    env::set_current_dir(temp_dir.path()).expect("Couldn't move to temp directory");
    copy_dir_contents(source, temp_dir.path()).expect("Could not copy template into directory");

    let perms = fs::Permissions::from_mode(0o777);
    fs::set_permissions(&temp_dir.path(), perms).expect("Failed to set permissions");

    temp_dir
}

fn sandbox_path() -> String {
    manifest_path() + "/src/tests/sandbox"
}

fn manifest_path() -> String {
    env!("CARGO_MANIFEST_DIR").to_string()
}