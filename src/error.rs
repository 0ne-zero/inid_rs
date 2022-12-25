use std::fmt;

pub enum INIDError{
    //InvalidChecksum,
    InvalidLength,
    //InvalidFormat,
    NotNumerical,
}

impl fmt::Display for INIDError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use INIDError::*;
        match self{
            //InvalidChecksum => write!(f,"We got \"InvalidChecksum\" Error"),
            //InvalidFormat => write!(f, "We got \"InvalidFormat\" Error"),
            InvalidLength => write!(f,"We got \"InvalidLength\" Error"),
            NotNumerical => write!(f,"We got \"NotNumerical\" Error"),
        }
    }
}

impl fmt::Debug for INIDError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use INIDError::*;
        match self{
            //InvalidChecksum => write!(f,"We got \"InvalidChecksum\" Error"),
            //InvalidFormat => write!(f, "We got \"InvalidFormat\" Error"),
            InvalidLength => write!(f,"We got \"InvalidLength\" Error"),
            NotNumerical => write!(f,"We got \"NotNumerical\" Error"),
        }
    }
}

impl std::error::Error for INIDError{}