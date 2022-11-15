// build.rs

use std::env;
extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // Copy our resources directory to OUT_DIR
    let mut options = CopyOptions::new();
    options.overwrite = true;
    copy(
        "../assets/",
        out_dir.to_str().unwrap().split("build").next().unwrap(),
        &options,
    )
    .unwrap();
}
