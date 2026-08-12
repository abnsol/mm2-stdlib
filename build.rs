use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_PYTHON");

    if env::var_os("CARGO_FEATURE_PYTHON").is_none() {
        return;
    }

    let output = Command::new("python3")
        .args([
            "-c",
            "import sysconfig; print(sysconfig.get_config_var('LIBPL') or '')",
        ])
        .output()
        .expect("failed to run python3 to locate libpython");

    if !output.status.success() {
        panic!("python3 exited unsuccessfully while locating libpython");
    }

    let libpl = String::from_utf8(output.stdout)
        .expect("python3 returned non-utf8 output while locating libpython")
        .trim()
        .to_owned();

    if libpl.is_empty() {
        panic!("python3 did not report a libpython search directory");
    }

    println!("cargo:rustc-link-search=native={libpl}");
}