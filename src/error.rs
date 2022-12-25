use std::fmt;

pub enum INIDError{
    InvalidChecksum,
    InvalidLength,
    InvalidFormat,
}

impl fmt::Display for INIDError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use INIDError::*;
        match self{
            InvalidChecksum => write!(f,"We got \"InvalidChecksum\" Error"),
            InvalidLength => write!(f,"We got \"InvalidLength\" Error"),
            InvalidFormat => write!(f, "We got \"InvalidFormat\" Error"),
        }
    }
}

impl fmt::Debug for INIDError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use INIDError::*;
        match self{
            InvalidChecksum => write!(f,"We got \"InvalidChecksum\" Error"),
            InvalidLength => write!(f,"We got \"InvalidLength\" Error"),
            InvalidFormat => write!(f, "We got \"InvalidFormat\" Error"),

        }
    }
}

impl std::error::Error for INIDError{}