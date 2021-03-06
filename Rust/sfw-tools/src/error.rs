use std::fmt::{self, Display};
use std::io::{Error, ErrorKind::*};
use std::option::NoneError;
use std::process;

const USER_ERROR_CODE: i32 = 1;

pub struct NoneErrorRich(NoneError);
const NONE_ERROR_RICH: NoneErrorRich = NoneErrorRich(NoneError);
//
impl fmt::Display for NoneErrorRich {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

pub fn user_exit(msg: &str) {
    eprintln!("{}", msg);
    process::exit(USER_ERROR_CODE)
}

pub trait SfwRes<T, E: Display> {
    fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T;

    /// Intended for use late in execution (e.g. in binaries),
    /// so that the program immediately exits with a user-friendly error message.
    fn user_err(self, fstr: &str) -> T
    where
        Self: Sized,
    {
        self.unwrap_or_else(|err| {
            eprintln!("{}: {}", fstr, err);
            process::exit(USER_ERROR_CODE)
        })
    }
}

impl<T, E: Display> SfwRes<T, E> for Result<T, E> {
    fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
        self.unwrap_or_else(op)
    }
}

impl<T> SfwRes<T, NoneErrorRich> for Option<T> {
    fn unwrap_or_else<F: FnOnce(NoneErrorRich) -> T>(self, op: F) -> T {
        self.unwrap_or_else(|| op(NONE_ERROR_RICH))
    }
}

pub trait SfwResError<T> {
    /// Intended as a potentially non-fatal error, typically
    /// used in library code.
    fn sfw_err(self, fstr: &str) -> Result<T, Error>
    where
        Self: Sized;
}

impl<T> SfwResError<T> for Result<T, Error> {
    fn sfw_err(self, fstr: &str) -> Result<T, Error> {
        self.map_err(|err| {
            Error::new(Error::kind(&err), format!("{}: {}", fstr, err))
        })
    }
}

impl<T> SfwResError<T> for Option<T> {
    fn sfw_err(self, fstr: &str) -> Result<T, Error> {
        match self {
            Some(s) => Ok(s),
            None => Err(Error::new(NotFound, fstr)),
        }
    }
}
