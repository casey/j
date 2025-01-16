use super::*;

pub(crate) trait PlatformInterface {
  /// translate path from "native" path to a path the interpreter expects
  fn convert_native_path(working_directory: &Path, path: &Path) -> FunctionResult;

  /// install handler, may only be called once
  fn install_signal_handler<T: Fn(Signal) + Send + 'static>(handler: T) -> RunResult<'static>;

  /// construct command equivalent to running script at `path` with shebang
  /// line `shebang`
  fn make_shebang_command(
    path: &Path,
    working_directory: Option<&Path>,
    shebang: Shebang,
  ) -> Result<Command, OutputError>;

  /// set the execute permission on file pointed to by `path`
  fn set_execute_permission(path: &Path) -> io::Result<()>;

  /// extract signal from process exit status
  fn signal_from_exit_status(exit_status: ExitStatus) -> Option<i32>;
}
