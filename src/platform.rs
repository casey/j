use super::*;

pub(crate) struct Platform;

#[cfg(unix)]
impl PlatformInterface for Platform {
  fn make_shebang_command(
    path: &Path,
    working_directory: Option<&Path>,
    _shebang: Shebang,
  ) -> Result<Command, OutputError> {
    // shebang scripts can be executed directly on unix
    let mut command = Command::new(path);

    if let Some(working_directory) = working_directory {
      command.current_dir(working_directory);
    }

    Ok(command)
  }

  fn set_execute_permission(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    // get current permissions
    let mut permissions = fs::metadata(path)?.permissions();

    // set the execute bit
    let current_mode = permissions.mode();
    permissions.set_mode(current_mode | 0o100);

    // set the new permissions
    fs::set_permissions(path, permissions)
  }

  fn signal_from_exit_status(exit_status: ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    exit_status.signal()
  }

  fn convert_native_path(_working_directory: &Path, path: &Path) -> FunctionResult {
    path
      .to_str()
      .map(str::to_string)
      .ok_or_else(|| String::from("Error getting current directory: unicode decode error"))
  }

  fn install_signal_handler<T: Fn(Signal) + Send + 'static>(handler: T) -> RunResult<'static> {
    thread::spawn(move || {
      // todo: handle errors?
      for signal in Signals::new().unwrap() {
        handler(signal.unwrap());
      }
    });
    Ok(())
  }
}

#[cfg(windows)]
impl PlatformInterface for Platform {
  fn make_shebang_command(
    path: &Path,
    working_directory: Option<&Path>,
    shebang: Shebang,
  ) -> Result<Command, OutputError> {
    use std::borrow::Cow;

    // If the path contains forward slashes…
    let command = if shebang.interpreter.contains('/') {
      // …translate path to the interpreter from unix style to windows style.
      let mut cygpath = Command::new("cygpath");
      if let Some(working_directory) = working_directory {
        cygpath.current_dir(working_directory);
      }
      cygpath.arg("--windows");
      cygpath.arg(shebang.interpreter);

      Cow::Owned(output(cygpath)?)
    } else {
      // …otherwise use it as-is.
      Cow::Borrowed(shebang.interpreter)
    };

    let mut cmd = Command::new(command.as_ref());

    if let Some(working_directory) = working_directory {
      cmd.current_dir(working_directory);
    }

    if let Some(argument) = shebang.argument {
      cmd.arg(argument);
    }

    cmd.arg(path);
    Ok(cmd)
  }

  fn set_execute_permission(_path: &Path) -> io::Result<()> {
    // it is not necessary to set an execute permission on a script on windows, so
    // this is a nop
    Ok(())
  }

  fn signal_from_exit_status(_exit_status: process::ExitStatus) -> Option<i32> {
    // The rust standard library does not expose a way to extract a signal from a
    // windows process exit status, so just return None
    None
  }

  fn convert_native_path(working_directory: &Path, path: &Path) -> FunctionResult {
    // Translate path from windows style to unix style
    let mut cygpath = Command::new("cygpath");
    cygpath.current_dir(working_directory);
    cygpath.arg("--unix");
    cygpath.arg(path);

    match output(cygpath) {
      Ok(shell_path) => Ok(shell_path),
      Err(_) => path
        .to_str()
        .map(str::to_string)
        .ok_or_else(|| String::from("Error getting current directory: unicode decode error")),
    }
  }

  fn install_signal_handler<T: Fn(Signal) + Send + 'static>(
    handler: T,
  ) -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| handler(Signal::Interrupt))?;
    Ok(())
  }
}
