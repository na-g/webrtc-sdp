use std::num::ParseIntError;
use std::net::AddrParseError;
use std::fmt;
use std::error;
use std::error::Error;

#[derive(Debug)]
pub enum SdpParserError {
    Line {
        message: String,
        line: String,
        //line_number: Option<usize>,
    },
    Unsupported {
        message: String,
        line: String,
        //line_number: Option<usize>,
    },
    Sequence {
        message: String,
        line_number: Option<usize>,
    },
    Integer(ParseIntError),
    Address(AddrParseError),
}

impl fmt::Display for SdpParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdpParserError::Line {
                ref message,
                ref line,
                ..
            } => write!(f, "Line error: {} in line: {}", message, line),
            SdpParserError::Unsupported {
                ref message,
                ref line,
                ..
            } => write!(f, "Unsupported: {} in line: {}", message, line),
            SdpParserError::Sequence { ref message, .. } => {
                write!(f, "Sequence error: {}", message)
            }
            SdpParserError::Integer(ref err) => {
                write!(f, "Integer parsing error: {}", err.description())
            }
            SdpParserError::Address(ref err) => {
                write!(f, "IP address parsing error: {}", err.description())
            }
        }
    }
}


impl error::Error for SdpParserError {
    fn description(&self) -> &str {
        match *self {
            SdpParserError::Line { ref message, .. } |
            SdpParserError::Unsupported { ref message, .. } |
            SdpParserError::Sequence { ref message, .. } => message,
            SdpParserError::Integer(ref err) => err.description(),
            SdpParserError::Address(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SdpParserError::Integer(ref err) => Some(err),
            SdpParserError::Address(ref err) => Some(err),
            // Can't tell much more about our internal errors
            _ => None,
        }
    }
}

impl From<ParseIntError> for SdpParserError {
    fn from(err: ParseIntError) -> SdpParserError {
        SdpParserError::Integer(err)
    }
}

impl From<AddrParseError> for SdpParserError {
    fn from(err: AddrParseError) -> SdpParserError {
        SdpParserError::Address(err)
    }
}

#[test]
fn test_sdp_parser_error_line() {
    let line = SdpParserError::Line {
        message: "test message".to_string(),
        line: "test line".to_string(),
        //line_number: None,
    };
    // TODO how to verify the output of fmt::Display() ?
    println!("{}", line);
    assert_eq!(line.description(), "test message");
    assert!(line.cause().is_none());
}

#[test]
fn test_sdp_parser_error_unsupported() {
    let unsupported = SdpParserError::Unsupported {
        message: "unsupported message".to_string(),
        line: "unsupported line".to_string(),
        //line_number: None,
    };
    // TODO how to verify the output of fmt::Display() ?
    println!("{}", unsupported);
    assert_eq!(unsupported.description(), "unsupported message");
    assert!(unsupported.cause().is_none());
}

#[test]
fn test_sdp_parser_error_sequence() {
    let sequence1 = SdpParserError::Sequence {
        message: "sequence message".to_string(),
        line_number: None,
    };
    // TODO how to verify the output of fmt::Display() ?
    println!("{}", sequence1);
    assert_eq!(sequence1.description(), "sequence message");
    assert!(sequence1.cause().is_none());

    let sequence2 = SdpParserError::Sequence {
        message: "another sequence message".to_string(),
        line_number: Some(5),
    };
    // TODO how to verify the output of fmt::Display() ?
    println!("{}", sequence2);
    assert_eq!(sequence2.description(), "another sequence message");
    assert!(sequence2.cause().is_none());
}

#[test]
fn test_sdp_parser_error_integer() {
    let v = "12a";
    let integer = v.parse::<u64>();
    assert!(integer.is_err());
    let int_err = SdpParserError::Integer(integer.err().unwrap());
    // TODO how to verify the output of fmt::Display() ?
    println!("{}", int_err);
    println!("{}", int_err.description());
    //assert!(int_err.cause().is_none());
}

#[test]
fn test_sdp_parser_error_address() {
    let v = "127.0.0.a";
    use std::str::FromStr;
    use std::net::IpAddr;
    let addr = IpAddr::from_str(v);
    assert!(addr.is_err());
    // TODO how to verify the output of fmt::Display() ?
    let addr_err = SdpParserError::Address(addr.err().unwrap());
    println!("{}", addr_err);
    println!("{}", addr_err.description());
    //assert!(addr_err.cause().is_none());
}
