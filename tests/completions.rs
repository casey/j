use super::*;

#[test]
#[cfg(linux)]
fn bash() {
  let output = Command::new(executable_path("just"))
    .args(["--completions", "bash"])
    .output()
    .unwrap();

  assert!(output.status.success());

  let script = str::from_utf8(&output.stdout).unwrap();

  let tempdir = tempdir();

  let path = tempdir.path().join("just.bash");

  fs::write(&path, &script).unwrap();

  let status = Command::new("./tests/completions/just.bash")
    .arg(path)
    .status()
    .unwrap();

  assert!(status.success());
}

#[test]
fn replacements() {
  for shell in ["bash", "elvish", "fish", "powershell", "zsh"] {
    let status = Command::new(executable_path("just"))
      .args(["--completions", shell])
      .status()
      .unwrap();
    assert!(status.success());
  }
}
