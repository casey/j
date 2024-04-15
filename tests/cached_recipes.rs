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

fn skipped_message(recipe_name: &str) -> String {
  format!(
    "===> Hash of recipe body of `{}` matches last run. Skipping...\n",
    recipe_name
  )
}

#[test]
fn cached_recipes_are_unstable() {
  let justfile = r#"
    [cached]
    echo:
      @echo cached
    "#;

  Test::new()
    .justfile(justfile)
    .stderr("error: Cached recipes are currently unstable. Invoke `just` with the `--unstable` flag to enable unstable features.\n")
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn cached_recipes_are_cached() {
  let justfile = r#"
    [cached]
    echo:
      @echo caching...
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper
    .map(|test| test.arg("--unstable").stdout("caching...\n"))
    .run();
  let _wrapper = wrapper
    .map(|test| test.arg("--unstable").stderr(skipped_message("echo")))
    .run();
}

#[test]
fn uncached_recipes_are_uncached() {
  let justfile = r#"
    echo:
      @echo uncached
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
    .map(|test| test.arg("--unstable").arg("echo1").stdout("cached1\n"))
    .run();
  let wrapper = wrapper
    .map(|test| test.arg("--unstable").arg("echo2").stdout("cached2\n"))
    .run();
  let wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .arg("echo1")
        .stderr(skipped_message("echo1"))
    })
    .run();
  let _wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .arg("echo2")
        .stderr(skipped_message("echo2"))
    })
    .run();
}

#[test]
fn interpolated_values_are_part_of_cache_hash() {
  let justfile = r#"
    my-var := "1"
    [cached]
    echo ARG:
      @echo {{ARG}}{{my-var}}
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper
    .map(|test| test.arg("--unstable").args(["echo", "a"]).stdout("a1\n"))
    .run();
  let wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .args(["echo", "a"])
        .stderr(skipped_message("echo"))
    })
    .run();
  let wrapper = wrapper
    .map(|test| test.arg("--unstable").args(["echo", "b"]).stdout("b1\n"))
    .run();
  let wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .args(["echo", "b"])
        .stderr(skipped_message("echo"))
    })
    .run();
  let wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .args(["my-var=2", "echo", "b"])
        .stdout("b2\n")
    })
    .run();
  let _wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .args(["my-var=2", "echo", "b"])
        .stderr(skipped_message("echo"))
    })
    .run();
}

#[test]
fn invalid_cache_files_are_ignored() {
  let justfile = r#"
    [cached]
    echo:
      @echo cached
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper
    .map(|test| test.arg("--unstable").stdout("cached\n"))
    .run();

  let cache_dir = wrapper.test.tempdir.path().join(".justcache");
  let mut caches = std::fs::read_dir(cache_dir).expect("could not read cache dir");
  let cached_recipe = caches.next().expect("no recipe cache file").unwrap().path();
  std::fs::write(cached_recipe, r#"{"invalid_cache_format": true}"#).unwrap();

  let _wrapper = wrapper
    .map(|test| test.arg("--unstable").stdout("cached\n"))
    .run();
}

#[test]
fn cached_deps_cannot_depend_on_preceding_uncached_ones() {
  let justfile = r#"
    [cached]
    cash-money: uncached
    uncached:
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let _wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .stderr(unindent(r#"
          error: Cached recipes cannot depend on preceding uncached ones, yet `cash-money` depends on `uncached`
           ——▶ justfile:2:13
            │
          2 │ cash-money: uncached
            │             ^^^^^^^^
          "#))
        .status(EXIT_FAILURE)
    })
    .run();
}

#[test]
fn subsequent_deps_run_only_when_cached_recipe_runs() {
  let justfile = r#"
    [cached]
    cash-money: && uncached
      @echo cash money
    uncached:
      @echo uncached cleanup
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .arg("cash-money")
        .stdout("cash money\nuncached cleanup\n")
    })
    .run();
  let _wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .arg("cash-money")
        .stderr(skipped_message("cash-money"))
    })
    .run();
}

#[test]
fn cached_recipes_rerun_when_deps_change_but_not_vice_versa() {
  let justfile = r#"
    top-var := "default-top"
    mid-var := "default-middle"
    bot-var := "default-bottom"

    [cached]
    top: mid
      @echo {{top-var}}
    [cached]
    mid: bot
      @echo {{mid-var}}
    [cached]
    bot:
      @echo {{bot-var}}
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper
    .map(|test| test.arg("--unstable").arg("bot").stdout("default-bottom\n"))
    .run();
  let wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .arg("top")
        .stderr(skipped_message("bot"))
        .stdout("default-middle\ndefault-top\n")
    })
    .run();

  let wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .args(["bot-var=change-bottom", "top"])
        .stdout("change-bottom\ndefault-middle\ndefault-top\n")
    })
    .run();

  let _wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .args(["bot-var=change-bottom", "top-var=change-top", "top"])
        .stderr([skipped_message("bot"), skipped_message("mid")].concat())
        .stdout("change-top\n")
    })
    .run();
}

#[test]
fn failed_runs_should_not_update_cache() {
  let justfile = r#"
    [cached]
    exit EXIT_CODE:
      @exit {{EXIT_CODE}}
    "#;

  let wrapper = ReuseableTest::new(justfile);
  let wrapper = wrapper
    .map(|test| test.arg("--unstable").args(["exit", "0"]))
    .run();
  let wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .args(["exit", "1"])
        .stderr("error: Recipe `exit` failed on line 3 with exit code 1\n")
        .status(EXIT_FAILURE)
    })
    .run();
  let _wrapper = wrapper
    .map(|test| {
      test
        .arg("--unstable")
        .args(["exit", "0"])
        .stderr(skipped_message("exit"))
    })
    .run();
}
