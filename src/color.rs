use {
  super::*,
  ansi_term::{ANSIGenericString, Color::*, Prefix, Style, Suffix},
  std::io::{self, IsTerminal},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Color {
  is_terminal: bool,
  style: Style,
  use_color: UseColor,
}

impl Color {
  pub(crate) fn active(&self) -> bool {
    match self.use_color {
      UseColor::Always => true,
      UseColor::Never => false,
      UseColor::Auto => self.is_terminal,
    }
  }

  pub(crate) fn alias(self) -> Self {
    self.restyle(Style::new().fg(Purple))
  }

  pub(crate) fn always() -> Self {
    Self {
      use_color: UseColor::Always,
      ..Self::default()
    }
  }

  pub(crate) fn annotation(self) -> Self {
    self.restyle(Style::new().fg(Purple))
  }

  pub(crate) fn auto() -> Self {
    Self::default()
  }

  pub(crate) fn banner(self) -> Self {
    self.restyle(Style::new().fg(Cyan).bold())
  }

  pub(crate) fn command(self, foreground: Option<ansi_term::Color>) -> Self {
    self.restyle(Style {
      foreground,
      is_bold: true,
      ..Style::default()
    })
  }

  pub(crate) fn context(self) -> Self {
    self.restyle(Style::new().fg(Blue).bold())
  }

  pub(crate) fn diff_added(self) -> Self {
    self.restyle(Style::new().fg(Green))
  }

  pub(crate) fn diff_deleted(self) -> Self {
    self.restyle(Style::new().fg(Red))
  }

  pub(crate) fn doc(self) -> Self {
    self.restyle(Style::new().fg(Blue))
  }

  pub(crate) fn doc_backtick(self) -> Self {
    self.restyle(Style::new().fg(Cyan))
  }

  fn effective_style(&self) -> Style {
    if self.active() {
      self.style
    } else {
      Style::new()
    }
  }

  pub(crate) fn error(self) -> Self {
    self.restyle(Style::new().fg(Red).bold())
  }

  pub(crate) fn group(self) -> Self {
    self.restyle(Style::new().fg(Yellow).bold())
  }

  pub(crate) fn message(self) -> Self {
    self.restyle(Style::new().bold())
  }

  pub(crate) fn never() -> Self {
    Self {
      use_color: UseColor::Never,
      ..Self::default()
    }
  }

  pub(crate) fn paint<'a>(&self, text: &'a str) -> ANSIGenericString<'a, str> {
    self.effective_style().paint(text)
  }

  pub(crate) fn parameter(self) -> Self {
    self.restyle(Style::new().fg(Cyan))
  }

  pub(crate) fn prefix(&self) -> Prefix {
    self.effective_style().prefix()
  }

  fn redirect(self, stream: impl IsTerminal) -> Self {
    Self {
      is_terminal: stream.is_terminal(),
      ..self
    }
  }

  fn restyle(self, style: Style) -> Self {
    Self { style, ..self }
  }

  pub(crate) fn stderr(self) -> Self {
    self.redirect(io::stderr())
  }

  pub(crate) fn stdout(self) -> Self {
    self.redirect(io::stdout())
  }

  pub(crate) fn string(self) -> Self {
    self.restyle(Style::new().fg(Green))
  }

  pub(crate) fn suffix(&self) -> Suffix {
    self.effective_style().suffix()
  }

  pub(crate) fn warning(self) -> Self {
    self.restyle(Style::new().fg(Yellow).bold())
  }
}

impl From<UseColor> for Color {
  fn from(use_color: UseColor) -> Self {
    Self {
      use_color,
      ..Default::default()
    }
  }
}

impl Default for Color {
  fn default() -> Self {
    Self {
      is_terminal: false,
      style: Style::new(),
      use_color: UseColor::Auto,
    }
  }
}
