use {
  super::*,
  nix::{
    errno::Errno,
    sys::signal::{SaFlags, SigAction, SigHandler, SigSet},
  },
};

static WRITE: AtomicI32 = AtomicI32::new(0);

fn die(message: &str) -> ! {
  const STDERR: BorrowedFd = unsafe { BorrowedFd::borrow_raw(libc::STDERR_FILENO) };

  nix::unistd::write(STDERR, b"just: ").ok();
  nix::unistd::write(STDERR, message.as_bytes()).ok();
  nix::unistd::write(STDERR, b"\n").ok();

  process::abort();
}

extern "C" fn handler(signal: libc::c_int) {
  let errno = Errno::last();

  let Ok(signal) = u8::try_from(signal) else {
    die("unexpected signal");
  };

  let buffer = &[signal as u8];

  let fd = unsafe { BorrowedFd::borrow_raw(WRITE.load(atomic::Ordering::Relaxed)) };

  if nix::unistd::write(fd, buffer).is_err() {
    die(Errno::last().desc());
  }

  errno.set();
}

pub(crate) struct Signals(File);

impl Signals {
  pub(crate) fn new() -> io::Result<Self> {
    let (read, write) = nix::unistd::pipe()?;

    if WRITE
      .compare_exchange(
        0,
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

    for signal in Signal::ALL {
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
