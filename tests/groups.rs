use super::*;

#[test]
fn list_with_groups() {
  Test::new()
    .justfile(
      "
        [group('alpha')]
        a:
        # Doc comment
        [group('alpha')]
        [group('beta')]
        b:
        c:
        [group('multi word group')]
        d:
        [group('alpha')]
        e:
        [group('beta')]
        [group('alpha')]
        f:
      ",
    )
    .arg("--list")
    .stdout(
      "
        Available recipes:
            (no group)
            c

            [alpha]
            a
            b # Doc comment
            e
            f

            [beta]
            b # Doc comment
            f

            [multi word group]
            d
      ",
    )
    .run();
}

#[test]
fn list_with_groups_unsorted() {
  Test::new()
    .justfile(
      "
        [group('beta')]
        [group('alpha')]
        f:

        [group('alpha')]
        e:

        [group('multi word group')]
        d:

        c:

        # Doc comment
        [group('alpha')]
        [group('beta')]
        b:

        [group('alpha')]
        a:

      ",
    )
    .args(["--list", "--unsorted"])
    .stdout(
      "
        Available recipes:
            (no group)
            c

            [alpha]
            f
            e
            b # Doc comment
            a

            [beta]
            f
            b # Doc comment

            [multi word group]
            d
      ",
    )
    .run();
}

#[test]
fn list_groups() {
  Test::new()
    .justfile(
      "
        [group('B')]
        bar:

        [group('A')]
        [group('B')]
        foo:

      ",
    )
    .args(["--groups"])
    .stdout(
      "
      Recipe groups:
          A
          B
      ",
    )
    .run();
}

#[test]
fn list_groups_with_custom_prefix() {
  Test::new()
    .justfile(
      "
        [group('B')]
        foo:

        [group('A')]
        [group('B')]
        bar:
      ",
    )
    .args(["--groups", "--list-prefix", "..."])
    .stdout(
      "
      Recipe groups:
      ...A
      ...B
      ",
    )
    .run();
}

#[test]
fn list_with_groups_in_modules() {
  Test::new()
    .justfile(
      "
        [group('FOO')]
        foo:

        mod bar
      ",
    )
    .write("bar.just", "[group('BAZ')]\nbaz:")
    .test_round_trip(false)
    .args(["--unstable", "--list"])
    .stdout(
      "
        Available recipes:
            [FOO]
            foo

            bar:
                [BAZ]
                baz
      ",
    )
    .run();
}
