use {
  super::*,
  nix::{
    errno::Errno,
    sys::signal::{SaFlags, SigAction, SigHandler, SigSet},
  },
};

static WRITE: AtomicI32 = AtomicI32::new(0);

extern "C" fn handler(signal: libc::c_int) {
  let last = Errno::last();

  let fd = WRITE.load(atomic::Ordering::Relaxed);

  let fd = unsafe { BorrowedFd::borrow_raw(fd) };

  if let Err(err) = nix::unistd::write(fd, &[signal as u8]) {
    // there are few times in life when one is well and truly fucked. this is
    // one.
  }

  last.set();
}

extern "C" fn handler_old(signal: libc::c_int) {
  let errno = unsafe { *libc::__error() };

  let buffer = &[signal as u8];

  let fd = WRITE.load(atomic::Ordering::Relaxed);

  unsafe {
    libc::write(fd, buffer.as_ptr().cast(), buffer.len());
  }

  // todo: should we abort if errno is bad?

  unsafe {
    *libc::__error() = errno;
  }
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
