use super::*;

fn build_test(justfile: &str, parent: &TempDir) -> Test {
  Test::with_tempdir(TempDir::new_in(&parent).expect("to create tempdir")).justfile(justfile)
}

#[test]
fn cached_recipes_are_actually_cached() {
  let justfile = r#"
    set cache-filename := "../local_cache.json"
    [cached]
    echo:
      @echo cached
    "#;
  let parent = tempdir();

  build_test(justfile, &parent).stdout("cached\n").run();
  build_test(justfile, &parent).stdout("").run();
}

#[test]
fn uncached_recipes_are_not_uncached() {
  let justfile = r#"
    set cache-filename := "../local_cache.json"
    @echo:
      echo uncached
    "#;
  let parent = tempdir();

  build_test(justfile, &parent).stdout("uncached\n").run();
  build_test(justfile, &parent).stdout("uncached\n").run();
}

#[test]
fn cached_recipes_are_independent() {
  let justfile = r#"
    set cache-filename := "../local_cache.json"

    [cached]
    echo1:
      @echo cached1

    [cached]
    echo2:
      @echo cached2
    "#;
  let parent = tempdir();

  build_test(justfile, &parent)
    .arg("echo1")
    .stdout("cached1\n")
    .run();
  build_test(justfile, &parent)
    .arg("echo2")
    .stdout("cached2\n")
    .run();
  build_test(justfile, &parent).arg("echo1").stdout("").run();
  build_test(justfile, &parent).arg("echo2").stdout("").run();
}

#[test]
fn arguments_and_variables_are_part_of_cache_hash() {
  let justfile = r#"
    set cache-filename := "../local_cache.json"
    my-var := "1"
    [cached]
    echo ARG:
      @echo {{ARG}}{{my-var}}
    "#;
  let parent = tempdir();

  build_test(justfile, &parent)
    .args(["echo", "a"])
    .stdout("a1\n")
    .run();
  build_test(justfile, &parent)
    .args(["echo", "a"])
    .stdout("")
    .run();
  build_test(justfile, &parent)
    .args(["echo", "b"])
    .stdout("b1\n")
    .run();
  build_test(justfile, &parent)
    .args(["echo", "b"])
    .stdout("")
    .run();
  build_test(justfile, &parent)
    .args(["my-var=2", "echo", "b"])
    .stdout("b2\n")
    .run();
  build_test(justfile, &parent)
    .args(["my-var=2", "echo", "b"])
    .stdout("")
    .run();
}

#[test]
fn invalid_recipe_errors() {
  let commands = [
    ("@echo {{`echo deja vu`}}", "a backtick expression"),
    ("@echo uuid4: {{uuid()}}", "a call to `uuid`"),
    ("@echo process_id: {{just_pid()}}", "a call to `just_pid`"),
  ];

  for (command, invalid) in commands {
    Test::new()
      .justfile(format!(
        r#"
        set cache-filename := "local_cache.json"
        [cached]
        invalid:
          {command}
        "#
      ))
      .stderr(format!("error: Cached recipe `invalid` contains {invalid}, which could run multiple times.\nYou must inline it if possible or set it to a variable outside of the recipe block: my_var := ...\n"))
      .status(EXIT_FAILURE)
      .run();
  }
}

#[test]
fn cached_recipes_rerun_when_deps_change_but_not_vice_versa() {
  assert!(false);
}

#[test]
fn cached_deps_cannot_depend_on_preceding_uncached_ones() {
  assert!(false);
}

#[test]
fn delete_cache_works() {
  assert!(false);
}

#[test]
fn default_cache_location_works() {
  assert!(false);
}
