use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=CSPROTUI_BASE_URL");

    let base_url = env::var("CSPROTUI_BASE_URL")
        .expect("CSPROTUI_BASE_URL environment variable must be set");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("base_url.rs");
    fs::write(
        &dest_path,
        format!(r#"pub const BASE_URL: &str = "{}";"#, base_url),
    )
    .unwrap();
}
