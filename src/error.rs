use std::error::Error as StdError;
use std::io;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
  Load(io::Error),
  Copy(io::Error, PathBuf, PathBuf),
  CopyTimeOut(PathBuf, PathBuf),
  Find(String)
}

impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Load(_) => "Unable to load library",
      Error::Copy(_, _, _) => "Unable to copy",
      Error::CopyTimeOut(_, _) => "Unable to copy due to time out",
      Error::Find(_) => "Unable to find",
    }
  }

  fn cause(&self) -> Option<&StdError> {
    match *self {
      Error::Load(ref e) => e.cause(),
      Error::Copy(ref e, _, _) => e.cause(),
      Error::CopyTimeOut(_, _) => None,
      Error::Find(_) => None,
    }
  }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
      match *self {
        Error::Load(ref e) => write!(fmt, "{} {}\nDue to: {:?}", self.description(), e.description(), self.cause()),
        Error::Copy(ref e, ref src, ref dest) => write!(fmt, "{} {:?} to {:?}\n{}\nDue to: {:?}", self.description(), src, dest, e.description(), self.cause()),
        Error::CopyTimeOut(ref src, ref dest) => write!(fmt, "{} {:?} to {:?}", self.description(), src, dest),
        Error::Find(ref name) => write!(fmt, "{} {}", self.description(), name),
      }
    }
}