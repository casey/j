use {
  super::*,
  nix::{
    errno::Errno,
    sys::signal::{SaFlags, SigAction, SigHandler, SigSet},
  },
};

const INVALID_FILENO: i32 = -1;

static WRITE: AtomicI32 = AtomicI32::new(INVALID_FILENO);

fn die(message: &str) -> ! {
  // SAFETY:
  //
  // Standard error is open for the duration of the program.
  const STDERR: BorrowedFd = unsafe { BorrowedFd::borrow_raw(libc::STDERR_FILENO) };

  nix::unistd::write(STDERR, b"error: ").ok();
  nix::unistd::write(STDERR, message.as_bytes()).ok();
  nix::unistd::write(STDERR, b"\n").ok();

  process::abort();
}

extern "C" fn handler(signal: libc::c_int) {
  let errno = Errno::last();

  let Ok(signal) = u8::try_from(signal) else {
    die("unexpected signal");
  };

  // SAFETY:
  //
  // `WRITE` is initialized before the signal handler can run and remains open
  // for the duration of the program.
  let fd = unsafe { BorrowedFd::borrow_raw(WRITE.load(atomic::Ordering::Relaxed)) };

  if let Err(err) = nix::unistd::write(fd, &[signal]) {
    die(err.desc());
  }

  errno.set();
}

pub(crate) struct Signals(File);

impl Signals {
  pub(crate) fn new() -> io::Result<Self> {
    let (read, write) = nix::unistd::pipe()?;

    if WRITE
      .compare_exchange(
        INVALID_FILENO,
        write.into_raw_fd(),
        atomic::Ordering::Relaxed,
        atomic::Ordering::Relaxed,
      )
      .is_err()
    {
      panic!("signal iterator cannot be initialized twice");
    }

    let sa = SigAction::new(
      SigHandler::Handler(handler),
      SaFlags::SA_RESTART,
      SigSet::empty(),
    );

    for &signal in Signal::ALL {
      // SAFETY:
      //
      // This is the only place we modify signal handlers, and
      // `nix::sys::signal::sigaction` is unsafe only if an invalid signal
      // handler has already been installed.
      unsafe {
        nix::sys::signal::sigaction(signal.into(), &sa)?;
      }
    }

    Ok(Self(File::from(read)))
  }
}

impl Iterator for Signals {
  type Item = io::Result<Signal>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut signal = [0];
    Some(
      self
        .0
        .read_exact(&mut signal)
        .and_then(|()| Signal::try_from(signal[0])),
    )
  }
}
