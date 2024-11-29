use super::*;

fn kill(process_id: u32) {
  nix::sys::signal::kill(
    nix::unistd::Pid::from_raw(process_id as i32),
    nix::sys::signal::Signal::SIGINT,
  )
  .unwrap();
}

fn interrupt_test(arguments: &[&str], justfile: &str) {
  let tmp = tempdir();
  let mut justfile_path = tmp.path().to_path_buf();
  justfile_path.push("justfile");
  fs::write(justfile_path, unindent(justfile)).unwrap();

  let start = Instant::now();

  let mut child = Command::new(executable_path("just"))
    .current_dir(&tmp)
    .args(arguments)
    .spawn()
    .expect("just invocation failed");

  while start.elapsed() < Duration::from_millis(500) {}

  kill(child.id());

  let status = child.wait().unwrap();

  let elapsed = start.elapsed();

  assert!(
    elapsed <= Duration::from_secs(2),
    "process returned too late: {elapsed:?}"
  );

  assert!(
    elapsed >= Duration::from_millis(100),
    "process returned too early : {elapsed:?}"
  );

  assert_eq!(status.code(), Some(130));
}

#[test]
#[ignore]
fn interrupt_shebang() {
  interrupt_test(
    &[],
    "
        default:
          #!/usr/bin/env sh
          sleep 1
      ",
  );
}

#[test]
#[ignore]
fn interrupt_line() {
  interrupt_test(
    &[],
    "
        default:
          @sleep 1
      ",
  );
}

#[test]
#[ignore]
fn interrupt_backtick() {
  interrupt_test(
    &[],
    "
        foo := `sleep 1`

        default:
          @echo {{foo}}
      ",
  );
}

#[test]
#[ignore]
fn interrupt_command() {
  interrupt_test(&["--command", "sleep", "1"], "");
}
