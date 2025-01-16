use super::*;

#[derive(Debug)]
pub(crate) enum OutputError {
  /// Non-zero exit code
  Code(i32),
  /// IO error
  Io(io::Error),
  /// Interrupted by signal
  Interrupted(Signal),
  /// Terminated by signal
  Signal(i32),
  /// Unknown failure
  Unknown,
  /// Stdout not UTF-8
  Utf8(str::Utf8Error),
}

impl OutputError {
  pub(crate) fn result_from_exit_status(exit_status: ExitStatus) -> Result<(), OutputError> {
    match exit_status.code() {
      Some(0) => Ok(()),
      Some(code) => Err(Self::Code(code)),
      None => match Platform::signal_from_exit_status(exit_status) {
        Some(signal) => Err(Self::Signal(signal)),
        None => Err(Self::Unknown),
      },
    }
  }
}

impl Display for OutputError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match *self {
      Self::Code(code) => write!(f, "Process exited with status code {code}"),
      Self::Io(ref io_error) => write!(f, "Error executing process: {io_error}"),
      Self::Interrupted(signal) => write!(
        f,
        "Process succeded but `just` was interrupted by signal {signal}"
      ),
      Self::Signal(signal) => write!(f, "Process terminated by signal {signal}"),
      Self::Unknown => write!(f, "Process experienced an unknown failure"),
      Self::Utf8(ref err) => write!(f, "Could not convert process stdout to UTF-8: {err}"),
    }
  }
}
