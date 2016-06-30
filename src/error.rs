use std::fmt;
pub type Result<T> = ::std::result::Result<T,Error>;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum Reason {
    NegativeMapLength,
    OutOfRange,
    AlreadyOccupied,
    MissingTarget,
    UnknownReason
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
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
            Reason::OutOfRange => {
                "want to access something that is not accessible"
            },
            Reason::MissingTarget => {
                "a target is required but was not found at this position"
            },
            Reason::AlreadyOccupied => {
                "this position is already occupied by something else"
            },
            Reason::UnknownReason => {
                "this should never happen"
            },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NegativeLengthError")
    }
}
