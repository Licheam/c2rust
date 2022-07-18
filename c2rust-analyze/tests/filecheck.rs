use std::env;
use std::fs;
use std::os::unix::io::{AsRawFd, FromRawFd};

use std::process::{Command, Stdio};


#[test]
fn filecheck() {
    let lib_dir = env::var("C2RUST_TARGET_LIB_DIR").unwrap();
    let lib_dir = &lib_dir;

    let filecheck_bin = env::var("FILECHECK").unwrap_or_else(|_| "FileCheck".into());

    for entry in fs::read_dir("tests/filecheck").unwrap() {
        let entry = entry.unwrap();

        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        let name = entry.file_name();
        let name = name.to_str().unwrap();
        if name.starts_with(".") || !name.ends_with(".rs") {
            continue;
        }

        eprintln!("{:?}", entry.path());

        let mut filecheck_cmd = Command::new(&filecheck_bin);
        filecheck_cmd
            .arg(entry.path())
            .stdin(Stdio::piped());
        let mut filecheck = filecheck_cmd.spawn().unwrap();
        let pipe_fd = filecheck.stdin.as_ref().unwrap().as_raw_fd();
        let mut analyze_cmd = Command::new("cargo");
        analyze_cmd
            .arg("run")
            .arg("--manifest-path").arg(format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")))
            .arg("--")
            .arg(entry.path())
            .arg("-L").arg(lib_dir)
            .arg("--crate-type").arg("rlib")
            .arg("--cfg").arg("compiling_for_test")
            .stdout(unsafe { Stdio::from_raw_fd(pipe_fd) })
            .stderr(unsafe { Stdio::from_raw_fd(pipe_fd) });
        let mut analyze = analyze_cmd.spawn().unwrap();

        let filecheck_status = filecheck.wait().unwrap();
        assert!(
            filecheck_status.success(),
            "{:?}: FileCheck failed with status {:?}",
            entry.path(), filecheck_status,
        );

        let analyze_status = analyze.wait().unwrap();
        assert!(
            analyze_status.success(),
            "{:?}: c2rust-analyze failed with status {:?}",
            entry.path(), analyze_status,
        );
    }
}
