use std::any::Any;
use std::fmt;
use std::io;
use std::sync::Mutex;

doc_rt_core! {
    /// Task failed to execute to completion.
    pub struct JoinError {
        repr: Repr,
    }
}

enum Repr {
    Cancelled,
    Panic(Mutex<Box<dyn Any + Send + 'static>>),
}

impl JoinError {
    #[doc(hidden)]
    #[deprecated]
    pub fn cancelled() -> JoinError {
        Self::cancelled2()
    }

    pub(crate) fn cancelled2() -> JoinError {
        JoinError {
            repr: Repr::Cancelled,
        }
    }

    #[doc(hidden)]
    #[deprecated]
    pub fn panic(err: Box<dyn Any + Send + 'static>) -> JoinError {
        Self::panic2(err)
    }

    pub(crate) fn panic2(err: Box<dyn Any + Send + 'static>) -> JoinError {
        JoinError {
            repr: Repr::Panic(Mutex::new(err)),
        }
    }

    ///Returns true iff the error was caused by the task being cancelled
    pub fn is_cancelled(&self) -> bool {
        match &self.repr {
            Repr::Cancelled => true,
            _ => false,
        }
    }

    ///Returns true iff the error was caused by the task panicking
    pub fn is_panic(&self) -> bool {
        match &self.repr {
            Repr::Panic(_) => true,
            _ => false,
        }
    }

    ///If the error was caused by a panic, consumes the error and
    /// returns the payload of the panic. Otherwise, returns an error
    /// result containing the original JoinError.
    pub fn into_panic(self) -> Result<Box<dyn Any + Send + 'static>, JoinError> {
        match self.repr {
            Repr::Panic(p) => Ok(p.into_inner().expect("Extracting panic from mutex")),
            _ => Err(self),
        }
    }
}

impl fmt::Display for JoinError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Cancelled => write!(fmt, "cancelled"),
            Repr::Panic(_) => write!(fmt, "panic"),
        }
    }
}

impl fmt::Debug for JoinError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Cancelled => write!(fmt, "JoinError::Cancelled"),
            Repr::Panic(_) => write!(fmt, "JoinError::Panic(...)"),
        }
    }
}

impl std::error::Error for JoinError {}

impl From<JoinError> for io::Error {
    fn from(src: JoinError) -> io::Error {
        io::Error::new(
            io::ErrorKind::Other,
            match src.repr {
                Repr::Cancelled => "task was cancelled",
                Repr::Panic(_) => "task panicked",
            },
        )
    }
}
