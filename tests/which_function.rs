use super::*;

#[test]
fn finds_executable() {
  let tmp = tempdir();
  let path = PathBuf::from(tmp.path());

  Test::with_tempdir(tmp)
    .justfile("p := which('hello.exe')")
    .args(["--evaluate", "p"])
    .write("hello.exe", "#!/usr/bin/env bash\necho hello\n")
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
    .write("hello.exe", "#!/usr/bin/env bash\necho hello\n")
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
    .write("hello.exe", "#!/usr/bin/env bash\necho hello\n")
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
    .write("subdir1/hello1.exe", "#!/usr/bin/env bash\necho hello\n")
    .make_executable("subdir1/hello1.exe")
    .write("subdir2/hello2.exe", "#!/usr/bin/env bash\necho hello\n")
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
      .write("dir1/shadowed.exe", "#!/usr/bin/env bash\necho hello\n")
      .make_executable("dir1/shadowed.exe")
      .write("dir2/shadowed.exe", "#!/usr/bin/env bash\necho hello\n")
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
    .write("subdir/foo.exe", "#!/usr/bin/env bash\necho hello\n")
    .make_executable("subdir/foo.exe")
    .write(dummy_exe, "#!/usr/bin/env bash\necho hello\n")
    .env("PATH", path_var.to_str().unwrap())
    .stdout(format!("{}", path.join("subdir").join("foo.exe").display()))
    .run();
}