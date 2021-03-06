//! Error types.

use crate::b64;
use core::fmt;

#[cfg(docsrs)]
use crate::PasswordHasher;

/// Password hash errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HashError {
    /// Hash output error.
    Hash(OutputError),

    /// Params error.
    Params(ParamsError),

    /// Parse error.
    Parse(ParseError),
}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Hash(err) => write!(f, "invalid password hash: {}", err),
            Self::Params(err) => write!(f, "invalid params: {}", err),
            Self::Parse(err) => write!(f, "parse error: {}", err),
        }
    }
}

impl From<OutputError> for HashError {
    fn from(err: OutputError) -> HashError {
        HashError::Hash(err)
    }
}

impl From<ParamsError> for HashError {
    fn from(err: ParamsError) -> HashError {
        match err {
            ParamsError::Parse(e) => e.into(),
            _ => HashError::Params(err),
        }
    }
}

impl From<ParseError> for HashError {
    fn from(err: ParseError) -> HashError {
        HashError::Parse(err)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HashError {}

/// Errors generating password hashes using a [`PasswordHasher`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HasherError {
    /// Unsupported algorithm.
    Algorithm,

    /// "B64" encoding error.
    B64(b64::Error),

    /// Cryptographic error.
    Crypto,

    /// Error generating output.
    Output(OutputError),

    /// Invalid parameter.
    Params(ParamsError),

    /// Parse error.
    Parse(ParseError),

    /// Invalid password.
    Password,
}

impl fmt::Display for HasherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Algorithm => write!(f, "unsupported algorithm"),
            Self::B64(err) => write!(f, "{}", err),
            Self::Crypto => write!(f, "cryptographic error"),
            Self::Output(err) => write!(f, "PHF output error: {}", err),
            Self::Params(err) => write!(f, "{}", err),
            Self::Parse(err) => write!(f, "{}", err),
            Self::Password => write!(f, "invalid password"),
        }
    }
}

impl From<b64::Error> for HasherError {
    fn from(err: b64::Error) -> HasherError {
        HasherError::B64(err)
    }
}

impl From<b64::InvalidLengthError> for HasherError {
    fn from(_: b64::InvalidLengthError) -> HasherError {
        HasherError::B64(b64::Error::InvalidLength)
    }
}

impl From<OutputError> for HasherError {
    fn from(err: OutputError) -> HasherError {
        HasherError::Output(err)
    }
}

impl From<ParamsError> for HasherError {
    fn from(err: ParamsError) -> HasherError {
        HasherError::Params(err)
    }
}

impl From<ParseError> for HasherError {
    fn from(err: ParseError) -> HasherError {
        HasherError::Parse(err)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HasherError {}

/// Parameter-related errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParamsError {
    /// Duplicate parameter name encountered.
    DuplicateName,

    /// Invalid parameter name.
    InvalidName,

    /// Invalid parameter value.
    InvalidValue,

    /// Maximum number of parameters exceeded.
    MaxExceeded,

    /// Parse errors.
    Parse(ParseError),
}

impl fmt::Display for ParamsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::DuplicateName => f.write_str("duplicate parameter"),
            Self::InvalidName => f.write_str("invalid parameter name"),
            Self::InvalidValue => f.write_str("invalid parameter value"),
            Self::MaxExceeded => f.write_str("maximum number of parameters reached"),
            Self::Parse(err) => write!(f, "{}", err),
        }
    }
}

impl From<ParseError> for ParamsError {
    fn from(err: ParseError) -> ParamsError {
        Self::Parse(err)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParamsError {}

/// Parse errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Invalid empty input.
    Empty,

    /// Input contains invalid character.
    InvalidChar(char),

    /// Input too short.
    TooShort,

    /// Input too long.
    TooLong,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Empty => f.write_str("invalid empty input"),
            Self::InvalidChar(char) => write!(f, "invalid character '{}'", char),
            Self::TooShort => f.write_str("too short"),
            Self::TooLong => f.write_str("too long"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

/// Password hash function output (i.e. hash/digest) errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OutputError {
    /// "B64" encoding error.
    B64(b64::Error),

    /// Output too short (min 10-bytes).
    TooShort,

    /// Output too long (max 64-bytes).
    TooLong,
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::B64(err) => write!(f, "{}", err),
            Self::TooShort => f.write_str("PHF output too short (min 10-bytes)"),
            Self::TooLong => f.write_str("PHF output too long (max 64-bytes)"),
        }
    }
}

impl From<b64::Error> for OutputError {
    fn from(err: b64::Error) -> OutputError {
        OutputError::B64(err)
    }
}

impl From<b64::InvalidLengthError> for OutputError {
    fn from(_: b64::InvalidLengthError) -> OutputError {
        OutputError::B64(b64::Error::InvalidLength)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OutputError {}

/// Password verification errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VerifyError;

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("password verification error")
    }
}

impl From<HasherError> for VerifyError {
    fn from(_: HasherError) -> VerifyError {
        VerifyError
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VerifyError {}
