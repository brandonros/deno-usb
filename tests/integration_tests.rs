use std::process::Command;
use std::path::PathBuf;

fn target_dir() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap();
  let target_dir = current_exe.parent().unwrap().parent().unwrap();
  println!("target_dir {}", target_dir.display());
  target_dir.into()
}

fn deno_cmd() -> Command {
  let mut deno_exe_path = target_dir().join("deno");
  if cfg!(windows) {
    deno_exe_path.set_extension("exe");
  }
  assert!(deno_exe_path.exists());
  Command::new(deno_exe_path)
}

#[cfg(debug_assertions)]
const BUILD_VARIANT: &str = "debug";

#[cfg(not(debug_assertions))]
const BUILD_VARIANT: &str = "release";

#[test]
fn basic() {
  let mut build_plugin_base = Command::new("cargo");
  let mut build_plugin =
    build_plugin_base.arg("build").arg("-p").arg("deno_usb");
  if BUILD_VARIANT == "release" {
    build_plugin = build_plugin.arg("--release");
  }
  let _build_plugin_output = build_plugin.output().unwrap();
  let output = deno_cmd()
    .arg("run")
    .arg("--allow-plugin")
    .arg("--unstable")
    .arg("tests/test.js")
    .arg(BUILD_VARIANT)
    .output()
    .unwrap();
  let stdout = std::str::from_utf8(&output.stdout).unwrap();
  let stderr = std::str::from_utf8(&output.stderr).unwrap();
  if !output.status.success() {
    println!("stdout {}", stdout);
    println!("stderr {}", stderr);
  }
  assert!(output.status.success());
  let expected = if cfg!(target_os = "windows") {
    "initContext response: 1\r\n"
  } else {
    "initContext response: 1\n"
  };
  assert_eq!(stdout, expected);
  assert_eq!(stderr, "");
}
