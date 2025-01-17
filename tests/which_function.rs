use super::*;

const HELLO_SCRIPT: &str = "#!/usr/bin/env bash
echo hello
";

#[test]
fn finds_executable() {
  let tmp = tempdir();
  let path = PathBuf::from(tmp.path());

  Test::with_tempdir(tmp)
    .justfile("p := which('hello.exe')")
    .args(["--evaluate", "p"])
    .write("hello.exe", HELLO_SCRIPT)
    .make_executable("hello.exe")
    .env("PATH", path.to_str().unwrap())
    .stdout(format!("{}", path.join("hello.exe").display()))
    .run();
}

#[test]
fn prints_empty_string_for_missing_executable() {
  let tmp = tempdir();
  let path = PathBuf::from(tmp.path());

  Test::with_tempdir(tmp)
    .justfile("p := which('goodbye.exe')")
    .args(["--evaluate", "p"])
    .write("hello.exe", HELLO_SCRIPT)
    .make_executable("hello.exe")
    .env("PATH", path.to_str().unwrap())
    .stdout("")
    .run();
}

#[test]
fn skips_non_executable_files() {
  let tmp = tempdir();
  let path = PathBuf::from(tmp.path());

  Test::with_tempdir(tmp)
    .justfile("p := which('hi')")
    .args(["--evaluate", "p"])
    .write("hello.exe", HELLO_SCRIPT)
    .make_executable("hello.exe")
    .write("hi", "just some regular file")
    .env("PATH", path.to_str().unwrap())
    .stdout("")
    .run();
}

#[test]
fn supports_multiple_paths() {
  let tmp = tempdir();
  let path = PathBuf::from(tmp.path());
  let path_var = env::join_paths([
    path.join("subdir1").to_str().unwrap(),
    path.join("subdir2").to_str().unwrap(),
  ])
  .unwrap();

  Test::with_tempdir(tmp)
    .justfile("p := which('hello1.exe') + '+' + which('hello2.exe')")
    .args(["--evaluate", "p"])
    .write("subdir1/hello1.exe", HELLO_SCRIPT)
    .make_executable("subdir1/hello1.exe")
    .write("subdir2/hello2.exe", HELLO_SCRIPT)
    .make_executable("subdir2/hello2.exe")
    .env("PATH", path_var.to_str().unwrap())
    .stdout(format!(
      "{}+{}",
      path.join("subdir1").join("hello1.exe").display(),
      path.join("subdir2").join("hello2.exe").display(),
    ))
    .run();
}

#[test]
fn supports_shadowed_executables() {
  enum Variation {
    Dir1Dir2, // PATH=/tmp/.../dir1:/tmp/.../dir2
    Dir2Dir1, // PATH=/tmp/.../dir2:/tmp/.../dir1
  }

  for variation in [Variation::Dir1Dir2, Variation::Dir2Dir1] {
    let tmp = tempdir();
    let path = PathBuf::from(tmp.path());

    let path_var = match variation {
      Variation::Dir1Dir2 => env::join_paths([
        path.join("dir1").to_str().unwrap(),
        path.join("dir2").to_str().unwrap(),
      ]),
      Variation::Dir2Dir1 => env::join_paths([
        path.join("dir2").to_str().unwrap(),
        path.join("dir1").to_str().unwrap(),
      ]),
    }
    .unwrap();

    let stdout = match variation {
      Variation::Dir1Dir2 => format!("{}", path.join("dir1").join("shadowed.exe").display()),
      Variation::Dir2Dir1 => format!("{}", path.join("dir2").join("shadowed.exe").display()),
    };

    Test::with_tempdir(tmp)
      .justfile("p := which('shadowed.exe')")
      .args(["--evaluate", "p"])
      .write("dir1/shadowed.exe", HELLO_SCRIPT)
      .make_executable("dir1/shadowed.exe")
      .write("dir2/shadowed.exe", HELLO_SCRIPT)
      .make_executable("dir2/shadowed.exe")
      .env("PATH", path_var.to_str().unwrap())
      .stdout(stdout)
      .run();
  }
}

#[test]
fn ignores_nonexecutable_candidates() {
  let tmp = tempdir();
  let path = PathBuf::from(tmp.path());

  let path_var = env::join_paths([
    path.join("dummy").to_str().unwrap(),
    path.join("subdir").to_str().unwrap(),
    path.join("dummy").to_str().unwrap(),
  ])
  .unwrap();

  let dummy_exe = if cfg!(windows) {
    "dummy/foo"
  } else {
    "dummy/foo.exe"
  };

  Test::with_tempdir(tmp)
    .justfile("p := which('foo.exe')")
    .args(["--evaluate", "p"])
    .write("subdir/foo.exe", HELLO_SCRIPT)
    .make_executable("subdir/foo.exe")
    .write(dummy_exe, HELLO_SCRIPT)
    .env("PATH", path_var.to_str().unwrap())
    .stdout(format!("{}", path.join("subdir").join("foo.exe").display()))
    .run();
}

#[test]
fn handles_absolute_path() {
  let tmp = tempdir();
  let path = PathBuf::from(tmp.path());
  let abspath = path.join("subdir").join("foo.exe");

  Test::with_tempdir(tmp)
    .justfile(format!("p := which('{}')", abspath.display()))
    .write("subdir/foo.exe", HELLO_SCRIPT)
    .make_executable("subdir/foo.exe")
    .write("pathdir/foo.exe", HELLO_SCRIPT)
    .make_executable("pathdir/foo.exe")
    .env("PATH", path.join("pathdir").to_str().unwrap())
    .args(["--evaluate", "p"])
    .stdout(format!("{}", abspath.display()))
    .run();
}

#[test]
fn handles_dotslash() {
  let tmp = tempdir();
  let path = tmp.path().canonicalize().unwrap();
  // canonicalize() is necessary here to account for the justfile prepending
  // the canonicalized working directory to './foo.exe'.

  Test::with_tempdir(tmp)
    .justfile("p := which('./foo.exe')")
    .args(["--evaluate", "p"])
    .write("foo.exe", HELLO_SCRIPT)
    .make_executable("foo.exe")
    .write("pathdir/foo.exe", HELLO_SCRIPT)
    .make_executable("pathdir/foo.exe")
    .env("PATH", path.join("pathdir").to_str().unwrap())
    .stdout(format!("{}", path.join("foo.exe").display()))
    .run();
}

#[test]
fn handles_dir_slash() {
  let tmp = tempdir();
  let path = tmp.path().canonicalize().unwrap();
  // canonicalize() is necessary here to account for the justfile prepending
  // the canonicalized working directory to 'subdir/foo.exe'.

  Test::with_tempdir(tmp)
    .justfile("p := which('subdir/foo.exe')")
    .args(["--evaluate", "p"])
    .write("subdir/foo.exe", HELLO_SCRIPT)
    .make_executable("subdir/foo.exe")
    .write("pathdir/foo.exe", HELLO_SCRIPT)
    .make_executable("pathdir/foo.exe")
    .env("PATH", path.join("pathdir").to_str().unwrap())
    .stdout(format!("{}", path.join("subdir").join("foo.exe").display()))
    .run();
}
