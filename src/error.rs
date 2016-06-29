use std::fmt;

#[derive(Debug,Copy,Clone)]
pub enum Reason {
    NegativeMapLength,
    UnknownReason
}

#[derive(Debug,Copy,Clone)]
pub struct Error {
    reason: Reason
}

impl Error {
    pub fn new(reason:Reason) -> Error {
        Error {
            reason: reason
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self.reason {
            Reason::NegativeMapLength => {
                "map was created with a negative length"
            },
            Reason::UnknownReason => {
                "this should never happen"
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NegativeLengthError")
    }
}
