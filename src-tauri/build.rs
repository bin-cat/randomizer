use std::{env, error::Error};

use fs_extra::{copy_items, dir::CopyOptions};

fn main() {
    tauri_build::build();

    println!("cargo:rerun-if-changed=../extra/bass.dll");
    println!("cargo:rerun-if-changed=../extra/data/*");
    println!("cargo:rerun-if-changed=../extra/plugins/*");

    let profile = env::var("PROFILE").expect("Failed to get profile");
    copy_to_output(
        &["../extra/bass.dll", "../extra/data", "../extra/plugins"],
        &profile,
    )
    .expect("Could not copy extra files");
}

fn copy_to_output(paths: &[&str], build_type: &str) -> Result<(), Box<dyn Error>> {
    let mut options = CopyOptions::new();

    options.overwrite = false;
    options.skip_exist = true;

    copy_items(paths, format!("../target/{build_type}"), &options)?;

    Ok(())
}
