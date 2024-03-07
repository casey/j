use super::*;

struct ReuseableTest {
  test: Test,
  justfile: &'static str,
}

impl ReuseableTest {
  pub(crate) fn new(justfile: &'static str) -> Self {
    Self {
      test: Test::new().justfile(justfile),
      justfile,
    }
  }

  fn new_with_test(justfile: &'static str, test: Test) -> Self {
    Self { test, justfile }
  }

  pub(crate) fn map(self, map: impl FnOnce(Test) -> Test) -> Self {
    Self::new_with_test(self.justfile, map(self.test))
  }

  pub(crate) fn run(self) -> Self {
    let justfile = self.justfile;
    let Output { tempdir, .. } = self.test.run();
    Self::new_with_test(justfile, Test::with_tempdir(tempdir).justfile(justfile))
  }
}

fn skipped_message<'run>(recipe_name: &str) -> String {
  format!(
    "===> Hash of recipe body of `{}` matches last run. Skipping...\n",
    recipe_name
  )
}

#[test]
fn cached_recipes_are_cached() {
  let justfile = r#"
    [cached]
    echo:
      @echo cached
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper.map(|test| test.stdout("cached\n")).run();
  let _wrapper = wrapper
    .map(|test| test.stderr(&skipped_message("echo")))
    .run();
}

#[test]
fn uncached_recipes_are_uncached() {
  let justfile = r#"
    @echo:
      echo uncached
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper.map(|test| test.stdout("uncached\n")).run();
  let _wrapper = wrapper.map(|test| test.stdout("uncached\n")).run();
}

#[test]
fn cached_recipes_are_independent() {
  let justfile = r#"

    [cached]
    echo1:
      @echo cached1

    [cached]
    echo2:
      @echo cached2
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper
    .map(|test| test.arg("echo1").stdout("cached1\n"))
    .run();
  let wrapper = wrapper
    .map(|test| test.arg("echo2").stdout("cached2\n"))
    .run();
  let wrapper = wrapper
    .map(|test| test.arg("echo1").stderr(&skipped_message("echo1")))
    .run();
  let _wrapper = wrapper
    .map(|test| test.arg("echo2").stderr(&skipped_message("echo2")))
    .run();
}

#[test]
fn arguments_and_variables_are_part_of_cache_hash() {
  let justfile = r#"
    my-var := "1"
    [cached]
    echo ARG:
      @echo {{ARG}}{{my-var}}
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper
    .map(|test| test.args(["echo", "a"]).stdout("a1\n"))
    .run();
  let wrapper = wrapper
    .map(|test| test.args(["echo", "a"]).stderr(&skipped_message("echo")))
    .run();
  let wrapper = wrapper
    .map(|test| test.args(["echo", "b"]).stdout("b1\n"))
    .run();
  let wrapper = wrapper
    .map(|test| test.args(["echo", "b"]).stderr(&skipped_message("echo")))
    .run();
  let wrapper = wrapper
    .map(|test| test.args(["my-var=2", "echo", "b"]).stdout("b2\n"))
    .run();
  let _wrapper = wrapper
    .map(|test| {
      test
        .args(["my-var=2", "echo", "b"])
        .stderr(&skipped_message("echo"))
    })
    .run();
}

#[test]
fn cached_recipes_rerun_when_deps_change_but_not_vice_versa() {
  assert!(false);
}

#[test]
fn cached_deps_cannot_depend_on_preceding_uncached_ones() {
  assert!(false);
}
