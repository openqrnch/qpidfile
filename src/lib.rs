//! A simple representation of a "pidfile".
//!
//! Creates a pidfile on creation and automatically remove it on termination.
//!
//! ```
//! fn main() {
//!   let pidfile = Pidfile::new("myserver.pid");
//!   // .. run server ..
//!
//!   // On termination the Pidfile will automatically be removed.
//! }
//! ```
//!
//! Be mindful of the [`Drop`] trait caveats; for instance calling
//! [`std::process::exit()`] will cause Drop traits not to run.
//!
//! [`std::process::exit()`]: https://doc.rust-lang.org/std/process/fn.exit.html
//! [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process;
use std::fs::File;

pub struct Pidfile {
  fname: PathBuf
}

impl Drop for Pidfile {
  fn drop(&mut self) {
    if let Err(e) = std::fs::remove_file(&self.fname) {
      eprintln!("Unable to remove pidfile {:?}; {}", self.fname, e);
    }
  }
}

/// Representation of a "pidfile", which contains the process identifier, of
/// the current process, in ascii base-10 format.
///
/// A [`Drop`] trait is used to automatically remove the pidfile on
/// termination.
///
/// [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
impl Pidfile {
  /// (Over)write the file specified in the parameter fname with the process
  /// idenfier of the current process.
  pub fn new<P: AsRef<Path>>(fname: P) -> std::io::Result<Self> {
    let mut file = File::create(fname.as_ref())?;
    let pidstr = format!("{}", process::id());
    file.write_all(pidstr.as_bytes())?;
    Ok(Pidfile { fname: fname.as_ref().to_path_buf() })
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
