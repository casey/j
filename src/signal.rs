use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub(crate) enum Signal {
  Hangup = libc::SIGHUP,
  #[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
  ))]
  Info = libc::SIGINFO,
  Interrupt = libc::SIGINT,
  Quit = libc::SIGQUIT,
  Terminate = libc::SIGTERM,
}

impl Signal {
  pub(crate) const ALL: [Signal; 5] = [
    Signal::Hangup,
    #[cfg(any(
      target_os = "dragonfly",
      target_os = "freebsd",
      target_os = "ios",
      target_os = "macos",
      target_os = "netbsd",
      target_os = "openbsd",
    ))]
    Signal::Info,
    Signal::Interrupt,
    Signal::Quit,
    Signal::Terminate,
  ];

  pub(crate) fn number(self) -> libc::c_int {
    self as libc::c_int
  }
}

impl Display for Signal {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Signal::Hangup => "SIGHUP",
        #[cfg(any(
          target_os = "dragonfly",
          target_os = "freebsd",
          target_os = "ios",
          target_os = "macos",
          target_os = "netbsd",
          target_os = "openbsd",
        ))]
        Signal::Info => "SIGINFO",
        Signal::Interrupt => "SIGINT",
        Signal::Quit => "SIGQUIT",
        Signal::Terminate => "SIGTERM",
      }
    )
  }
}

impl From<Signal> for nix::sys::signal::Signal {
  fn from(signal: Signal) -> Self {
    match signal {
      Signal::Hangup => Self::SIGHUP,
      #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
      ))]
      Signal::Info => Self::SIGINFO,
      Signal::Interrupt => Self::SIGINT,
      Signal::Quit => Self::SIGQUIT,
      Signal::Terminate => Self::SIGTERM,
    }
  }
}

impl TryFrom<u8> for Signal {
  type Error = io::Error;

  fn try_from(n: u8) -> Result<Signal, Self::Error> {
    match n.into() {
      libc::SIGHUP => Ok(Signal::Hangup),
      #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
      ))]
      libc::SIGINFO => Ok(Signal::Info),
      libc::SIGINT => Ok(Signal::Interrupt),
      libc::SIGQUIT => Ok(Signal::Quit),
      libc::SIGTERM => Ok(Signal::Terminate),
      _ => Err(io::Error::new(
        io::ErrorKind::Other,
        format!("unexpected signal: {n}"),
      )),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn signals_fit_in_u8() {
    for signal in Signal::ALL {
      assert!(signal.number() <= i32::from(u8::MAX));
    }
  }
}
