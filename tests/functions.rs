use super::*;

test! {
  name:     test_os_arch_functions_in_interpolation,
  justfile: r"
foo:
  echo {{arch()}} {{os()}} {{os_family()}} {{num_cpus()}}
",
  stdout:   format!("{} {} {} {}\n", target::arch(), target::os(), target::family(), num_cpus::get()).as_str(),
  stderr:   format!("echo {} {} {} {}\n", target::arch(), target::os(), target::family(), num_cpus::get()).as_str(),
}

test! {
  name:     test_os_arch_functions_in_expression,
  justfile: r"
a := arch()
o := os()
f := os_family()
n := num_cpus()

foo:
  echo {{a}} {{o}} {{f}} {{n}}
",
  stdout:   format!("{} {} {} {}\n", target::arch(), target::os(), target::family(), num_cpus::get()).as_str(),
  stderr:   format!("echo {} {} {} {}\n", target::arch(), target::os(), target::family(), num_cpus::get()).as_str(),
}

#[cfg(not(windows))]
test! {
  name:     env_var_functions,
  justfile: r"
p := env_var('USER')
b := env_var_or_default('ZADDY', 'HTAP')
x := env_var_or_default('XYZ', 'ABC')

foo:
  /usr/bin/env echo '{{p}}' '{{b}}' '{{x}}'
",
  stdout:   format!("{} HTAP ABC\n", env::var("USER").unwrap()).as_str(),
  stderr:   format!("/usr/bin/env echo '{}' 'HTAP' 'ABC'\n", env::var("USER").unwrap()).as_str(),
}

#[cfg(not(windows))]
test! {
  name: path_functions,
  justfile: r"
we  := without_extension('/foo/bar/baz.hello')
fs  := file_stem('/foo/bar/baz.hello')
fn  := file_name('/foo/bar/baz.hello')
dir := parent_directory('/foo/bar/baz.hello')
ext := extension('/foo/bar/baz.hello')
jn  := join('a', 'b')

foo:
  /usr/bin/env echo '{{we}}' '{{fs}}' '{{fn}}' '{{dir}}' '{{ext}}' '{{jn}}'
",
  stdout:   "/foo/bar/baz baz baz.hello /foo/bar hello a/b\n",
  stderr:   "/usr/bin/env echo '/foo/bar/baz' 'baz' 'baz.hello' '/foo/bar' 'hello' 'a/b'\n",
}

#[cfg(not(windows))]
test! {
  name: path_functions2,
  justfile: r"
we  := without_extension('/foo/bar/baz')
fs  := file_stem('/foo/bar/baz.hello.ciao')
fn  := file_name('/bar/baz.hello.ciao')
dir := parent_directory('/foo/')
ext := extension('/foo/bar/baz.hello.ciao')

foo:
  /usr/bin/env echo '{{we}}' '{{fs}}' '{{fn}}' '{{dir}}' '{{ext}}'
",
  stdout:   "/foo/bar/baz baz.hello baz.hello.ciao / ciao\n",
  stderr:   "/usr/bin/env echo '/foo/bar/baz' 'baz.hello' 'baz.hello.ciao' '/' 'ciao'\n",
}

#[cfg(not(windows))]
test! {
  name: broken_without_extension_function,
  justfile: r"
we  := without_extension('')

foo:
  /usr/bin/env echo '{{we}}'
",
  stdout:   "",
  stderr:   format!("{} {}\n{}\n{}\n{}\n{}\n",
    "error: Call to function `without_extension` failed:",
    "Could not extract parent from ``",
    " ——▶ justfile:1:8",
    "  │",
    "1 │ we  := without_extension(\'\')",
    "  │        ^^^^^^^^^^^^^^^^^").as_str(),
  status:   EXIT_FAILURE,
}

#[cfg(not(windows))]
test! {
  name: broken_extension_function,
  justfile: r"
we  := extension('')

foo:
  /usr/bin/env echo '{{we}}'
",
  stdout:   "",
  stderr:   format!("{}\n{}\n{}\n{}\n{}\n",
    "error: Call to function `extension` failed: Could not extract extension from ``",
    " ——▶ justfile:1:8",
    "  │",
    "1 │ we  := extension(\'\')",
    "  │        ^^^^^^^^^").as_str(),
  status:   EXIT_FAILURE,
}

#[cfg(not(windows))]
test! {
  name: broken_extension_function2,
  justfile: r"
we  := extension('foo')

foo:
  /usr/bin/env echo '{{we}}'
",
  stdout:   "",
  stderr:   format!("{}\n{}\n{}\n{}\n{}\n",
    "error: Call to function `extension` failed: Could not extract extension from `foo`",
    " ——▶ justfile:1:8",
    "  │",
    "1 │ we  := extension(\'foo\')",
    "  │        ^^^^^^^^^").as_str(),
  status:   EXIT_FAILURE,
}

#[cfg(not(windows))]
test! {
  name: broken_file_stem_function,
  justfile: r"
we  := file_stem('')

foo:
  /usr/bin/env echo '{{we}}'
",
  stdout:   "",
  stderr:   format!("{}\n{}\n{}\n{}\n{}\n",
    "error: Call to function `file_stem` failed: Could not extract file stem from ``",
    " ——▶ justfile:1:8",
    "  │",
    "1 │ we  := file_stem(\'\')",
    "  │        ^^^^^^^^^").as_str(),
  status:   EXIT_FAILURE,
}

#[cfg(not(windows))]
test! {
  name: broken_file_name_function,
  justfile: r"
we  := file_name('')

foo:
  /usr/bin/env echo '{{we}}'
",
  stdout:   "",
  stderr:   format!("{}\n{}\n{}\n{}\n{}\n",
    "error: Call to function `file_name` failed: Could not extract file name from ``",
    " ——▶ justfile:1:8",
    "  │",
    "1 │ we  := file_name(\'\')",
    "  │        ^^^^^^^^^").as_str(),
  status:   EXIT_FAILURE,
}

#[cfg(not(windows))]
test! {
  name: broken_directory_function,
  justfile: r"
we  := parent_directory('')

foo:
  /usr/bin/env echo '{{we}}'
",
  stdout:   "",
  stderr:   format!("{} {}\n{}\n{}\n{}\n{}\n",
    "error: Call to function `parent_directory` failed:",
    "Could not extract parent directory from ``",
    " ——▶ justfile:1:8",
    "  │",
    "1 │ we  := parent_directory(\'\')",
    "  │        ^^^^^^^^^^^^^^^^").as_str(),
  status:   EXIT_FAILURE,
}

#[cfg(not(windows))]
test! {
  name: broken_directory_function2,
  justfile: r"
we  := parent_directory('/')

foo:
  /usr/bin/env echo '{{we}}'
",
  stdout:   "",
  stderr:   format!("{} {}\n{}\n{}\n{}\n{}\n",
    "error: Call to function `parent_directory` failed:",
    "Could not extract parent directory from `/`",
    " ——▶ justfile:1:8",
    "  │",
    "1 │ we  := parent_directory(\'/\')",
    "  │        ^^^^^^^^^^^^^^^^").as_str(),
  status:   EXIT_FAILURE,
}

#[cfg(windows)]
test! {
  name:     env_var_functions,
  justfile: r#"
p := env_var('USERNAME')
b := env_var_or_default('ZADDY', 'HTAP')
x := env_var_or_default('XYZ', 'ABC')

foo:
  /usr/bin/env echo '{{p}}' '{{b}}' '{{x}}'
"#,
  stdout:   format!("{} HTAP ABC\n", env::var("USERNAME").unwrap()).as_str(),
  stderr:   format!("/usr/bin/env echo '{}' 'HTAP' 'ABC'\n", env::var("USERNAME").unwrap()).as_str(),
}

test! {
  name:     env_var_failure,
  justfile: "a:\n  echo {{env_var('ZADDY')}}",
  args:     ("a"),
  stdout:   "",
  stderr:   "error: Call to function `env_var` failed: environment variable `ZADDY` not present
 ——▶ justfile:2:10
  │
2 │   echo {{env_var('ZADDY')}}
  │          ^^^^^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     test_just_executable_function,
  justfile: "
    a:
      @printf 'Executable path is: %s\\n' '{{ just_executable() }}'
  ",
  args:     ("a"),
  stdout:   format!("Executable path is: {}\n", executable_path("just").to_str().unwrap()).as_str(),
  stderr:   "",
  status:   EXIT_SUCCESS,
}

test! {
  name:     test_os_arch_functions_in_default,
  justfile: r"
foo a=arch() o=os() f=os_family() n=num_cpus():
  echo {{a}} {{o}} {{f}} {{n}}
",
  stdout:   format!("{} {} {} {}\n", target::arch(), target::os(), target::family(), num_cpus::get()).as_str(),
  stderr:   format!("echo {} {} {} {}\n", target::arch(), target::os(), target::family(), num_cpus::get()).as_str(),
}

test! {
  name: clean,
  justfile: "
    foo:
      echo {{ clean('a/../b') }}
  ",
  stdout: "b\n",
  stderr: "echo b\n",
}

test! {
  name: uppercase,
  justfile: "
    foo:
      echo {{ uppercase('bar') }}
  ",
  stdout: "BAR\n",
  stderr: "echo BAR\n",
}

test! {
  name: lowercase,
  justfile: "
    foo:
      echo {{ lowercase('BAR') }}
  ",
  stdout: "bar\n",
  stderr: "echo bar\n",
}

test! {
  name: uppercamelcase,
  justfile: "
    foo:
      echo {{ uppercamelcase('foo bar') }}
  ",
  stdout: "FooBar\n",
  stderr: "echo FooBar\n",
}

test! {
  name: lowercamelcase,
  justfile: "
    foo:
      echo {{ lowercamelcase('foo bar') }}
  ",
  stdout: "fooBar\n",
  stderr: "echo fooBar\n",
}

test! {
  name: snakecase,
  justfile: "
    foo:
      echo {{ snakecase('foo bar') }}
  ",
  stdout: "foo_bar\n",
  stderr: "echo foo_bar\n",
}

test! {
  name: kebabcase,
  justfile: "
    foo:
      echo {{ kebabcase('foo bar') }}
  ",
  stdout: "foo-bar\n",
  stderr: "echo foo-bar\n",
}

test! {
  name: shoutysnakecase,
  justfile: "
    foo:
      echo {{ shoutysnakecase('foo bar') }}
  ",
  stdout: "FOO_BAR\n",
  stderr: "echo FOO_BAR\n",
}

test! {
  name: titlecase,
  justfile: "
    foo:
      echo {{ titlecase('foo bar') }}
  ",
  stdout: "Foo Bar\n",
  stderr: "echo Foo Bar\n",
}

test! {
  name: shoutykebabcase,
  justfile: "
    foo:
      echo {{ shoutykebabcase('foo bar') }}
  ",
  stdout: "FOO-BAR\n",
  stderr: "echo FOO-BAR\n",
}

test! {
  name: trim,
  justfile: "
    foo:
      echo {{ trim('   bar   ') }}
  ",
  stdout: "bar\n",
  stderr: "echo bar\n",
}

test! {
  name: replace,
  justfile: "
    foo:
      echo {{ replace('barbarbar', 'bar', 'foo') }}
  ",
  stdout: "foofoofoo\n",
  stderr: "echo foofoofoo\n",
}

test! {
  name: replace_regex,
  justfile: "
    foo:
      echo {{ replace_regex('123bar123bar123bar', '\\d+bar', 'foo') }}
  ",
  stdout: "foofoofoo\n",
  stderr: "echo foofoofoo\n",
}

test! {
  name: invalid_replace_regex,
  justfile: "
    foo:
      echo {{ replace_regex('barbarbar', 'foo\\', 'foo') }}
  ",
  stderr:
"error: Call to function `replace_regex` failed: regex parse error:
    foo\\
       ^
error: incomplete escape sequence, reached end of pattern prematurely
 ——▶ justfile:2:11
  │
2 │   echo {{ replace_regex('barbarbar', 'foo\\', 'foo') }}
  │           ^^^^^^^^^^^^^
",
  status: EXIT_FAILURE,
}

test! {
    name: capitalize,
    justfile: "
      foo:
        echo {{ capitalize('BAR') }}
    ",
    stdout: "Bar\n",
    stderr: "echo Bar\n",
}

#[test]
fn semver_matches() {
  Test::new()
    .justfile(
      "
      foo:
        echo {{ semver_matches('0.1.0', '>=0.1.0') }}
        echo {{ semver_matches('0.1.0', '=0.0.1') }}
    ",
    )
    .stdout("true\nfalse\n")
    .stderr("echo true\necho false\n")
    .run();
}

#[test]
fn trim_end_matches() {
  assert_eval_eq("trim_end_matches('foo', 'o')", "f");
  assert_eval_eq("trim_end_matches('fabab', 'ab')", "f");
  assert_eval_eq("trim_end_matches('fbaabab', 'ab')", "fba");
}

#[test]
fn trim_end_match() {
  assert_eval_eq("trim_end_match('foo', 'o')", "fo");
  assert_eval_eq("trim_end_match('fabab', 'ab')", "fab");
}

#[test]
fn trim_start_matches() {
  assert_eval_eq("trim_start_matches('oof', 'o')", "f");
  assert_eval_eq("trim_start_matches('ababf', 'ab')", "f");
  assert_eval_eq("trim_start_matches('ababbaf', 'ab')", "baf");
}

#[test]
fn trim_start_match() {
  assert_eval_eq("trim_start_match('oof', 'o')", "of");
  assert_eval_eq("trim_start_match('ababf', 'ab')", "abf");
}

#[test]
fn trim_start() {
  assert_eval_eq("trim_start('  f  ')", "f  ");
}

#[test]
fn trim_end() {
  assert_eval_eq("trim_end('  f  ')", "  f");
}

#[test]
fn append() {
  assert_eval_eq("append('8', 'r s t')", "r8 s8 t8");
  assert_eval_eq("append('.c', 'main sar x11')", "main.c sar.c x11.c");
  assert_eval_eq("append('-', 'c v h y')", "c- v- h- y-");
  assert_eval_eq(
    "append('0000', '11 10 01 00')",
    "110000 100000 010000 000000",
  );
  assert_eval_eq(
    "append('tion', '
    Determina
    Acquisi
    Motiva
    Conjuc
    ')",
    "Determination Acquisition Motivation Conjuction",
  );
}

#[test]
fn prepend() {
  assert_eval_eq("prepend('8', 'r s t\n  \n  ')", "8r 8s 8t");
  assert_eval_eq(
    "prepend('src/', 'main sar x11')",
    "src/main src/sar src/x11",
  );
  assert_eval_eq("prepend('-', 'c\tv h\ny')", "-c -v -h -y");
  assert_eval_eq(
    "prepend('0000', '11 10 01 00')",
    "000011 000010 000001 000000",
  );
  assert_eval_eq(
    "prepend('April-', '
      1st,
        17th,
    20th,
    ')",
    "April-1st, April-17th, April-20th,",
  );
}

#[test]
#[cfg(not(windows))]
fn join() {
  assert_eval_eq("join('a', 'b', 'c', 'd')", "a/b/c/d");
  assert_eval_eq("join('a', '/b', 'c', 'd')", "/b/c/d");
  assert_eval_eq("join('a', '/b', '/c', 'd')", "/c/d");
  assert_eval_eq("join('a', '/b', '/c', '/d')", "/d");
}

#[test]
#[cfg(windows)]
fn join() {
  assert_eval_eq("join('a', 'b', 'c', 'd')", "a\\b\\c\\d");
  assert_eval_eq("join('a', '\\b', 'c', 'd')", "\\b\\c\\d");
  assert_eval_eq("join('a', '\\b', '\\c', 'd')", "\\c\\d");
  assert_eval_eq("join('a', '\\b', '\\c', '\\d')", "\\d");
}

#[test]
fn join_argument_count_error() {
  Test::new()
    .justfile("x := join('a')")
    .args(["--evaluate"])
    .stderr(
      "
      error: Function `join` called with 1 argument but takes 2 or more
       ——▶ justfile:1:6
        │
      1 │ x := join(\'a\')
        │      ^^^^
      ",
    )
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn test_path_exists_filepath_exist() {
  Test::new()
    .tree(tree! {
      testfile: ""
    })
    .justfile("x := path_exists('testfile')")
    .args(["--evaluate", "x"])
    .stdout("true")
    .run();
}

#[test]
fn test_path_exists_filepath_doesnt_exist() {
  Test::new()
    .justfile("x := path_exists('testfile')")
    .args(["--evaluate", "x"])
    .stdout("false")
    .run();
}

#[test]
fn error_errors_with_message() {
  Test::new()
    .justfile("x := error ('Thing Not Supported')")
    .args(["--evaluate"])
    .status(1)
    .stderr(
      "
      error: Call to function `error` failed: Thing Not Supported
       ——▶ justfile:1:6
        │
      1 │ x := error ('Thing Not Supported')
        │      ^^^^^
    ",
    )
    .run();
}

#[test]
fn test_absolute_path_resolves() {
  let test_object = Test::new()
    .justfile("path := absolute_path('./test_file')")
    .args(["--evaluate", "path"]);

  let mut tempdir = test_object.tempdir.path().to_owned();

  // Just retrieves the current directory via env::current_dir(), which
  // does the moral equivalent of canonicalize, which will remove symlinks.
  // So, we have to canonicalize here, so that we can match it.
  if cfg!(unix) {
    tempdir = tempdir.canonicalize().unwrap();
  }

  test_object
    .stdout(tempdir.join("test_file").to_str().unwrap().to_owned())
    .run();
}

#[test]
fn test_absolute_path_resolves_parent() {
  let test_object = Test::new()
    .justfile("path := absolute_path('../test_file')")
    .args(["--evaluate", "path"]);

  let mut tempdir = test_object.tempdir.path().to_owned();

  // Just retrieves the current directory via env::current_dir(), which
  // does the moral equivalent of canonicalize, which will remove symlinks.
  // So, we have to canonicalize here, so that we can match it.
  if cfg!(unix) {
    tempdir = tempdir.canonicalize().unwrap();
  }

  test_object
    .stdout(
      tempdir
        .parent()
        .unwrap()
        .join("test_file")
        .to_str()
        .unwrap()
        .to_owned(),
    )
    .run();
}

#[test]
fn path_exists_subdir() {
  Test::new()
    .tree(tree! {
      foo: "",
      bar: {
      }
    })
    .justfile("x := path_exists('foo')")
    .current_dir("bar")
    .args(["--evaluate", "x"])
    .stdout("true")
    .run();
}

#[test]
fn uuid() {
  Test::new()
    .justfile("x := uuid()")
    .args(["--evaluate", "x"])
    .stdout_regex("........-....-....-....-............")
    .run();
}

#[test]
fn choose() {
  Test::new()
    .justfile(r"x := choose('10', 'xXyYzZ')")
    .args(["--evaluate", "x"])
    .stdout_regex("^[X-Zx-z]{10}$")
    .run();
}

#[test]
fn choose_bad_alphabet_empty() {
  Test::new()
    .justfile("x := choose('10', '')")
    .args(["--evaluate"])
    .status(1)
    .stderr(
      "
      error: Call to function `choose` failed: empty alphabet
       ——▶ justfile:1:6
        │
      1 │ x := choose('10', '')
        │      ^^^^^^
    ",
    )
    .run();
}

#[test]
fn choose_bad_alphabet_repeated() {
  Test::new()
    .justfile("x := choose('10', 'aa')")
    .args(["--evaluate"])
    .status(1)
    .stderr(
      "
      error: Call to function `choose` failed: alphabet contains repeated character `a`
       ——▶ justfile:1:6
        │
      1 │ x := choose('10', 'aa')
        │      ^^^^^^
    ",
    )
    .run();
}

#[test]
fn choose_bad_length() {
  Test::new()
    .justfile("x := choose('foo', HEX)")
    .args(["--evaluate"])
    .status(1)
    .stderr(
      "
      error: Call to function `choose` failed: failed to parse `foo` as positive integer: invalid digit found in string
       ——▶ justfile:1:6
        │
      1 │ x := choose('foo', HEX)
        │      ^^^^^^
    ",
    )
    .run();
}

#[test]
fn sha256() {
  Test::new()
    .justfile("x := sha256('5943ee37-0000-1000-8000-010203040506')")
    .args(["--evaluate", "x"])
    .stdout("2330d7f5eb94a820b54fed59a8eced236f80b633a504289c030b6a65aef58871")
    .run();
}

#[test]
fn sha256_file() {
  Test::new()
    .justfile("x := sha256_file('sub/shafile')")
    .tree(tree! {
      sub: {
        shafile: "just is great\n",
      }
    })
    .current_dir("sub")
    .args(["--evaluate", "x"])
    .stdout("177b3d79aaafb53a7a4d7aaba99a82f27c73370e8cb0295571aade1e4fea1cd2")
    .run();
}

#[test]
fn just_pid() {
  let Output { stdout, pid, .. } = Test::new()
    .args(["--evaluate", "x"])
    .justfile("x := just_pid()")
    .stdout_regex(r"\d+")
    .run();

  assert_eq!(stdout.parse::<u32>().unwrap(), pid);
}

#[test]
fn shell_no_argument() {
  Test::new()
    .justfile("var := shell()")
    .args(["--evaluate"])
    .stderr(
      "
      error: Function `shell` called with 0 arguments but takes 1 or more
       ——▶ justfile:1:8
        │
      1 │ var := shell()
        │        ^^^^^
      ",
    )
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn shell_minimal() {
  assert_eval_eq("shell('echo $1 $2', 'justice', 'legs')", "justice legs");
}

#[test]
fn shell_args() {
  assert_eval_eq("shell('echo $@', 'justice', 'legs')", "justice legs");
}

#[test]
fn shell_first_arg() {
  assert_eval_eq("shell('echo $0')", "echo $0");
}

#[test]
fn shell_error() {
  Test::new()
    .justfile("var := shell('exit 1')")
    .args(["--evaluate"])
    .stderr(
      "
      error: Call to function `shell` failed: Process exited with status code 1
       ——▶ justfile:1:8
        │
      1 │ var := shell('exit 1')
        │        ^^^^^
      ",
    )
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn blake3() {
  Test::new()
    .justfile("x := blake3('5943ee37-0000-1000-8000-010203040506')")
    .args(["--evaluate", "x"])
    .stdout("026c9f740a793ff536ddf05f8915ea4179421f47f0fa9545476076e9ba8f3f2b")
    .run();
}

#[test]
fn blake3_file() {
  Test::new()
    .justfile("x := blake3_file('sub/blakefile')")
    .tree(tree! {
      sub: {
        blakefile: "just is great\n",
      }
    })
    .current_dir("sub")
    .args(["--evaluate", "x"])
    .stdout("8379241877190ca4b94076a8c8f89fe5747f95c62f3e4bf41f7408a0088ae16d")
    .run();
}

#[cfg(unix)]
#[test]
fn canonicalize() {
  Test::new()
    .args(["--evaluate", "x"])
    .justfile("x := canonicalize('foo')")
    .symlink("justfile", "foo")
    .stdout_regex(".*/justfile")
    .run();
}

#[test]
fn encode_uri_component() {
  Test::new()
    .justfile("x := encode_uri_component(\"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\\\"#$%&'()*+,-./:;<=>?@[\\\\]^_`{|}~ \\t\\r\\n🌐\")")
    .args(["--evaluate", "x"])
    .stdout("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!%22%23%24%25%26'()*%2B%2C-.%2F%3A%3B%3C%3D%3E%3F%40%5B%5C%5D%5E_%60%7B%7C%7D~%20%09%0D%0A%F0%9F%8C%90")
    .run();
}

#[test]
fn source_file() {
  Test::new()
    .args(["--evaluate", "x"])
    .justfile("x := source_file()")
    .stdout_regex(r".*[/\\]justfile")
    .run();

  Test::new()
    .args(["--evaluate", "x"])
    .justfile(
      "
        import 'foo.just'
      ",
    )
    .write("foo.just", "x := source_file()")
    .stdout_regex(r".*[/\\]foo.just")
    .run();

  Test::new()
    .args(["foo", "bar"])
    .justfile(
      "
        mod foo
      ",
    )
    .write("foo.just", "x := source_file()\nbar:\n @echo '{{x}}'")
    .stdout_regex(r".*[/\\]foo.just\n")
    .run();
}

#[test]
fn source_directory() {
  Test::new()
    .args(["foo", "bar"])
    .justfile(
      "
        mod foo
      ",
    )
    .write(
      "foo/mod.just",
      "x := source_directory()\nbar:\n @echo '{{x}}'",
    )
    .stdout_regex(r".*[/\\]foo\n")
    .run();
}

#[test]
fn module_paths() {
  Test::new()
    .write(
      "foo/bar.just",
      "
imf := module_file()
imd := module_directory()

import-outer: import-inner

@import-inner pmf=module_file() pmd=module_directory():
  echo import
  echo '{{ imf }}'
  echo '{{ imd }}'
  echo '{{ pmf }}'
  echo '{{ pmd }}'
  echo '{{ module_file() }}'
  echo '{{ module_directory() }}'
      ",
    )
    .write(
      "baz/mod.just",
      "
import 'foo/bar.just'

mmf := module_file()
mmd := module_directory()

outer: inner

@inner pmf=module_file() pmd=module_directory():
  echo module
  echo '{{ mmf }}'
  echo '{{ mmd }}'
  echo '{{ pmf }}'
  echo '{{ pmd }}'
  echo '{{ module_file() }}'
  echo '{{ module_directory() }}'
      ",
    )
    .write(
      "baz/foo/bar.just",
      "
imf := module_file()
imd := module_directory()

import-outer: import-inner

@import-inner pmf=module_file() pmd=module_directory():
  echo import
  echo '{{ imf }}'
  echo '{{ imd }}'
  echo '{{ pmf }}'
  echo '{{ pmd }}'
  echo '{{ module_file() }}'
  echo '{{ module_directory() }}'
      ",
    )
    .justfile(
      "
        import 'foo/bar.just'
        mod baz

        rmf := module_file()
        rmd := module_directory()

        outer: inner

        @inner pmf=module_file() pmd=module_directory():
          echo root
          echo '{{ rmf }}'
          echo '{{ rmd }}'
          echo '{{ pmf }}'
          echo '{{ pmd }}'
          echo '{{ module_file() }}'
          echo '{{ module_directory() }}'
      ",
    )
    .args([
      "outer",
      "import-outer",
      "baz",
      "outer",
      "baz",
      "import-outer",
    ])
    .stdout_regex(
      r"root
.*[/\\]just-test-tempdir......[/\\]justfile
.*[/\\]just-test-tempdir......
.*[/\\]just-test-tempdir......[/\\]justfile
.*[/\\]just-test-tempdir......
.*[/\\]just-test-tempdir......[/\\]justfile
.*[/\\]just-test-tempdir......
import
.*[/\\]just-test-tempdir......[/\\]justfile
.*[/\\]just-test-tempdir......
.*[/\\]just-test-tempdir......[/\\]justfile
.*[/\\]just-test-tempdir......
.*[/\\]just-test-tempdir......[/\\]justfile
.*[/\\]just-test-tempdir......
module
.*[/\\]just-test-tempdir......[/\\]baz[/\\]mod.just
.*[/\\]just-test-tempdir......[/\\]baz
.*[/\\]just-test-tempdir......[/\\]baz[/\\]mod.just
.*[/\\]just-test-tempdir......[/\\]baz
.*[/\\]just-test-tempdir......[/\\]baz[/\\]mod.just
.*[/\\]just-test-tempdir......[/\\]baz
import
.*[/\\]just-test-tempdir......[/\\]baz[/\\]mod.just
.*[/\\]just-test-tempdir......[/\\]baz
.*[/\\]just-test-tempdir......[/\\]baz[/\\]mod.just
.*[/\\]just-test-tempdir......[/\\]baz
.*[/\\]just-test-tempdir......[/\\]baz[/\\]mod.just
.*[/\\]just-test-tempdir......[/\\]baz
",
    )
    .run();
}

#[test]
fn is_dependency() {
  let justfile = "
    alpha: beta
      @echo 'alpha {{is_dependency()}}'
    beta: && gamma
      @echo 'beta {{is_dependency()}}'
    gamma:
      @echo 'gamma {{is_dependency()}}'
  ";
  Test::new()
    .args(["alpha"])
    .justfile(justfile)
    .stdout("beta true\ngamma true\nalpha false\n")
    .run();

  Test::new()
    .args(["beta"])
    .justfile(justfile)
    .stdout("beta false\ngamma true\n")
    .run();
}

#[test]
fn unary_argument_count_mismamatch_error_message() {
  Test::new()
    .justfile("x := datetime()")
    .args(["--evaluate"])
    .stderr(
      "
      error: Function `datetime` called with 0 arguments but takes 1
       ——▶ justfile:1:6
        │
      1 │ x := datetime()
        │      ^^^^^^^^
      ",
    )
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn dir_abbreviations_are_accepted() {
  Test::new()
    .justfile(
      "
      abbreviated := justfile_dir()
      unabbreviated := justfile_directory()

      @foo:
        # {{ assert(abbreviated == unabbreviated, 'fail') }}
    ",
    )
    .run();
}

#[test]
fn invocation_dir_native_abbreviation_is_accepted() {
  Test::new()
    .justfile(
      "
      abbreviated := invocation_directory_native()
      unabbreviated := invocation_dir_native()

      @foo:
        # {{ assert(abbreviated == unabbreviated, 'fail') }}
    ",
    )
    .run();
}

#[test]
fn absolute_path_argument_is_relative_to_submodule_working_directory() {
  Test::new()
    .justfile("mod foo")
    .write("foo/baz", "")
    .write(
      "foo/mod.just",
      r#"
bar:
  @echo "{{ absolute_path('baz') }}"

"#,
    )
    .stdout_regex(r".*[/\\]foo[/\\]baz\n")
    .args(["foo", "bar"])
    .run();
}

#[test]
fn blake3_file_argument_is_relative_to_submodule_working_directory() {
  Test::new()
    .justfile("mod foo")
    .write("foo/baz", "")
    .write(
      "foo/mod.just",
      "
bar:
  @echo {{ blake3_file('baz') }}

",
    )
    .stdout("af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262\n")
    .args(["foo", "bar"])
    .run();
}

#[test]
fn canonicalize_argument_is_relative_to_submodule_working_directory() {
  Test::new()
    .justfile("mod foo")
    .write("foo/baz", "")
    .write(
      "foo/mod.just",
      r#"
bar:
  @echo "{{ canonicalize('baz') }}"

"#,
    )
    .stdout_regex(r".*[/\\]foo[/\\]baz\n")
    .args(["foo", "bar"])
    .run();
}

#[test]
fn path_exists_argument_is_relative_to_submodule_working_directory() {
  Test::new()
    .justfile("mod foo")
    .write("foo/baz", "")
    .write(
      "foo/mod.just",
      "
bar:
  @echo {{ path_exists('baz') }}

",
    )
    .stdout_regex("true\n")
    .args(["foo", "bar"])
    .run();
}

#[test]
fn sha256_file_argument_is_relative_to_submodule_working_directory() {
  Test::new()
    .justfile("mod foo")
    .write("foo/baz", "")
    .write(
      "foo/mod.just",
      "
bar:
  @echo {{ sha256_file('baz') }}

",
    )
    .stdout_regex("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n")
    .args(["foo", "bar"])
    .run();
}

#[test]
fn style_command_default() {
  Test::new()
    .justfile(
      r#"
        foo:
          @echo '{{ style("command") }}foo{{NORMAL}}'
      "#,
    )
    .stdout("\x1b[1mfoo\x1b[0m\n")
    .run();
}

#[test]
fn style_command_non_default() {
  Test::new()
    .justfile(
      r#"
        foo:
          @echo '{{ style("command") }}foo{{NORMAL}}'
      "#,
    )
    .args(["--command-color", "red"])
    .stdout("\x1b[1;31mfoo\x1b[0m\n")
    .run();
}

#[test]
fn style_error() {
  Test::new()
    .justfile(
      r#"
        foo:
          @echo '{{ style("error") }}foo{{NORMAL}}'
      "#,
    )
    .stdout("\x1b[1;31mfoo\x1b[0m\n")
    .run();
}

#[test]
fn style_warning() {
  Test::new()
    .justfile(
      r#"
        foo:
          @echo '{{ style("warning") }}foo{{NORMAL}}'
      "#,
    )
    .stdout("\x1b[1;33mfoo\x1b[0m\n")
    .run();
}

#[test]
fn style_unknown() {
  Test::new()
    .justfile(
      r#"
        foo:
          @echo '{{ style("hippo") }}foo{{NORMAL}}'
      "#,
    )
    .stderr(
      r#"
        error: Call to function `style` failed: unknown style: `hippo`
         ——▶ justfile:2:13
          │
        2 │   @echo '{{ style("hippo") }}foo{{NORMAL}}'
          │             ^^^^^
      "#,
    )
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn read() {
  Test::new()
    .justfile("foo := read('bar')")
    .write("bar", "baz")
    .args(["--evaluate", "foo"])
    .stdout("baz")
    .run();
}

#[test]
fn read_file_not_found() {
  Test::new()
    .justfile("foo := read('bar')")
    .args(["--evaluate", "foo"])
    .stderr_regex(r"error: Call to function `read` failed: I/O error reading `bar`: .*")
    .status(EXIT_FAILURE)
    .run();
}
