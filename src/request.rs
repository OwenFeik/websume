use std::{
    collections::HashMap, ffi::OsStr, os::unix::ffi::OsStrExt, path::PathBuf,
};

/// Errors that can be encountered while parsing an HTTP request.
#[derive(Debug)]
pub enum ParseFailure {
    InvalidHttpVerb,
    InvalidPathByte(u8),
    PathTooLong,
    UnsupportedHttpVersion,
    MissingNewline,
    InvalidHeaderByte(u8),
    HeaderNotValidUtf8,
    InvalidHeaderSyntax(usize),
    HeaderTooLong,
}

/// Potential outcomes of parsing a buffer of bytes as part of an HTTP request.
/// `Ongoing` if the request is incomplete, `Complete` if the request was
/// finished by the provided buffer and `Failed` on a parse error.
pub enum ParseOutcome {
    Ongoing(RequestParser),
    Complete(HttpRequest),
    Failed(ParseFailure),
}

/// Parses request data into a [HttpRequest].
pub struct RequestParser(ParseStep);

impl RequestParser {
    /// Create a fresh [RequestParser].
    pub fn new() -> Self {
        Self(ParseStep::Verb { data: Vec::new() })
    }

    /// Parse the provided `bytes` as part of HTTP request, returning an outcome
    /// describing the state of the parse.
    pub fn parse(self, bytes: &[u8]) -> ParseOutcome {
        match self.0.parse(bytes) {
            Err(error) => ParseOutcome::Failed(error),
            Ok(ParseStep::Body(request)) => ParseOutcome::Complete(request),
            Ok(step) => ParseOutcome::Ongoing(Self(step)),
        }
    }
}

/// Valid HTTP request verbs.
#[derive(Debug)]
pub enum HttpVerb {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Query,
    Trace,
}

impl HttpVerb {
    /// Return the enum representation of the provided HTTP verb byte string.
    fn from_bytes(data: &[u8]) -> Result<Self, ParseFailure> {
        match data {
            b"CONNECT" => Ok(Self::Connect),
            b"DELETE" => Ok(Self::Delete),
            b"GET" => Ok(Self::Get),
            b"HEAD" => Ok(Self::Head),
            b"OPTIONS" => Ok(Self::Patch),
            b"POST" => Ok(Self::Post),
            b"PUT" => Ok(Self::Put),
            b"QUERY" => Ok(Self::Query),
            b"TRACE" => Ok(Self::Trace),
            _ => Err(ParseFailure::InvalidHttpVerb),
        }
    }
}

/// Supported HTTP versions.
#[derive(Debug)]
enum HttpVersion {
    PointNine,
    OnePointZero,
    OnePointOne,
}

/// An HTTP request, including headers but not body.
#[derive(Debug)]
pub struct HttpRequest {
    pub verb: HttpVerb,
    pub path: PathBuf,
    version: HttpVersion,
    pub headers: HttpHeaders,
}

/// Headers from an [HttpRequest].
#[derive(Debug)]
pub struct HttpHeaders(HashMap<String, String>);

impl HttpHeaders {
    /// Create a new empty set of headers.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Retrieve the value of the provided header, if any.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
        self.0.get(key.as_ref()).map(|string| string.as_str())
    }

    /// Add a header to this header map.
    pub fn insert(&mut self, key: impl ToString, value: String) {
        self.0.insert(key.to_string(), value);
    }

    /// Number of headers present.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

type ParseStepResult = Result<ParseStep, ParseFailure>;

/// State of parsing an HTTP header line.
#[derive(Debug, Default)]
struct HeaderParseState {
    header_name_data: Vec<u8>,
    header_value_data: Vec<u8>,
    passed_colon: bool,
    passed_space: bool,
    passed_return: bool,
}

/// States involved in passing an HTTP request.
#[derive(Debug)]
enum ParseStep {
    Verb {
        data: Vec<u8>,
    },
    Path {
        verb: HttpVerb,
        data: Vec<u8>,
    },
    Version {
        verb: HttpVerb,
        path: PathBuf,
        data: Vec<u8>,
    },
    Newline(HttpRequest),
    Headers {
        request: HttpRequest,
        state: HeaderParseState,
    },
    Body(HttpRequest),
}

impl ParseStep {
    /// Parse the provided bytes, returning the updated parse step after
    /// processing all bytes, or an error.
    fn parse(mut self, bytes: &[u8]) -> ParseStepResult {
        for byte in bytes {
            self = self.parse_byte(*byte)?;
        }
        Ok(self)
    }

    /// Parse a single byte, returning the updated parse step or an error.
    fn parse_byte(self, byte: u8) -> ParseStepResult {
        match self {
            Self::Verb { data } => parse_verb_byte(data, byte),
            Self::Path { verb, data } => parse_path_byte(verb, data, byte),
            Self::Version { verb, path, data } => {
                parse_version_byte(verb, path, data, byte)
            }
            Self::Newline(request) => parse_first_newline(request, byte),
            Self::Headers { request, state } => {
                parse_headers_byte(request, state, byte)
            }
            Self::Body(request) => Ok(Self::Body(request)), // Discard body bytes for now.
        }
    }
}

/// Parse one byte of the HTTP verb. Will advance to [ParseStep::Path] after
/// successfully parsing a full verb.
fn parse_verb_byte(mut data: Vec<u8>, byte: u8) -> ParseStepResult {
    const MAX_VERB_LENGTH: usize = 7; // CONNECT / OPTIONS
    match byte {
        b' ' => {
            let verb = HttpVerb::from_bytes(&data)?;
            data.clear();
            Ok(ParseStep::Path { verb, data })
        }
        _ => {
            data.push(byte);
            if data.len() > MAX_VERB_LENGTH {
                Err(ParseFailure::InvalidHttpVerb)
            } else {
                Ok(ParseStep::Verb { data })
            }
        }
    }
}

fn parse_path_byte(
    verb: HttpVerb,
    mut data: Vec<u8>,
    byte: u8,
) -> ParseStepResult {
    const MAX_PATH_LENGTH: usize = 1024;
    if byte == b' ' {
        let path = PathBuf::from(OsStr::from_bytes(&data));
        data.clear();
        Ok(ParseStep::Version { verb, path, data })
    } else if is_valid_path_char(byte) {
        data.push(byte);
        if data.len() > MAX_PATH_LENGTH {
            Err(ParseFailure::PathTooLong)
        } else {
            Ok(ParseStep::Path { verb, data })
        }
    } else {
        Err(ParseFailure::InvalidPathByte(byte))
    }
}

fn is_valid_path_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || byte == b'-'
        || byte == b'.'
        || byte == b'_'
        || byte == b'~'
        || byte == b':'
        || byte == b'/'
        || byte == b'?'
        || byte == b'#'
        || byte == b'['
        || byte == b']'
        || byte == b'@'
        || byte == b'!'
        || byte == b'$'
        || byte == b'&'
        || byte == b'\''
        || byte == b'('
        || byte == b')'
        || byte == b'*'
        || byte == b'+'
        || byte == b','
        || byte == b';'
        || byte == b'%'
        || byte == b'='
}

fn parse_version_byte(
    verb: HttpVerb,
    path: PathBuf,
    mut data: Vec<u8>,
    byte: u8,
) -> ParseStepResult {
    const MAX_VERSION_LENGTH: usize = "HTTP/1.1".len();

    if byte == b'\r' {
        let version = match data.as_slice() {
            b"HTTP/0.9" => HttpVersion::PointNine,
            b"HTTP/1.0" => HttpVersion::OnePointZero,
            b"HTTP/1.1" => HttpVersion::OnePointOne,
            _ => return Err(ParseFailure::UnsupportedHttpVersion),
        };
        Ok(ParseStep::Newline(HttpRequest {
            verb,
            path,
            version,
            headers: HttpHeaders::new(),
        }))
    } else {
        if b"HTTP/091.".contains(&byte) {
            data.push(byte);
            if data.len() > MAX_VERSION_LENGTH {
                Err(ParseFailure::UnsupportedHttpVersion)
            } else {
                Ok(ParseStep::Version { verb, path, data })
            }
        } else {
            Err(ParseFailure::UnsupportedHttpVersion)
        }
    }
}

fn parse_first_newline(request: HttpRequest, byte: u8) -> ParseStepResult {
    if byte == b'\n' {
        Ok(ParseStep::Headers {
            request,
            state: Default::default(),
        })
    } else {
        Err(ParseFailure::MissingNewline)
    }
}

fn parse_headers_byte(
    mut request: HttpRequest,
    mut state: HeaderParseState,
    byte: u8,
) -> ParseStepResult {
    const MAX_HEADER_NAME_LENGTH: usize = 256;
    const MAX_HEADER_VALUE_LENGTH: usize = 1024;

    if state.passed_return {
        if byte == b'\n'
            && state.header_name_data.is_empty()
            && !state.passed_colon
            && !state.passed_space
            && state.header_value_data.is_empty()
        {
            Ok(ParseStep::Body(request))
        } else if state.passed_colon && state.passed_space {
            if let Ok(name) = String::from_utf8(state.header_name_data)
                && let Ok(value) = String::from_utf8(state.header_value_data)
            {
                request.headers.insert(name, value);
                Ok(ParseStep::Headers {
                    request,
                    state: Default::default(),
                })
            } else {
                Err(ParseFailure::HeaderNotValidUtf8)
            }
        } else {
            Err(ParseFailure::InvalidHeaderByte(byte))
        }
    } else if state.passed_colon && !state.passed_space {
        if byte == b' ' {
            state.passed_space = true;
            Ok(ParseStep::Headers { request, state })
        } else {
            Err(ParseFailure::InvalidHeaderSyntax(request.headers.len()))
        }
    } else if state.passed_space {
        match byte {
            b'\r' => {
                state.passed_return = true;
                Ok(ParseStep::Headers { request, state })
            }
            b'\n' | b'\0' => Err(ParseFailure::InvalidHeaderByte(byte)),
            _ => {
                state.header_value_data.push(byte);
                if state.header_value_data.len() > MAX_HEADER_VALUE_LENGTH {
                    Err(ParseFailure::HeaderTooLong)
                } else {
                    Ok(ParseStep::Headers { request, state })
                }
            }
        }
    } else {
        if valid_header_name_byte(byte) {
            state.header_name_data.push(byte);
            if state.header_name_data.len() > MAX_HEADER_NAME_LENGTH {
                Err(ParseFailure::HeaderTooLong)
            } else {
                Ok(ParseStep::Headers { request, state })
            }
        } else if byte == b':' {
            state.passed_colon = true;
            Ok(ParseStep::Headers { request, state })
        } else if byte == b'\r' {
            state.passed_return = true;
            Ok(ParseStep::Headers { request, state })
        } else {
            Err(ParseFailure::InvalidHeaderByte(byte))
        }
    }
}

/// Test if provided byte is valid as part of an HTTP header.
fn valid_header_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || byte == b'!'
        || byte == b'#'
        || byte == b'$'
        || byte == b'%'
        || byte == b'&'
        || byte == b'\''
        || byte == b'*'
        || byte == b'+'
        || byte == b'-'
        || byte == b'.'
        || byte == b'^'
        || byte == b'_'
        || byte == b'`'
        || byte == b'|'
        || byte == b'~'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_request() {
        let req = concat!(
            "GET /path/to/../../some/sneaky/file HTTP/1.1\r\n",
            "Content-Length: 12\r\n",
            "Content-Type: nonsense\r\n\r\n",
            "aaaaaaaaaaaa"
        );

        let request = match RequestParser::new().parse(req.as_bytes()) {
            ParseOutcome::Ongoing(_) => panic!("parser exited early"),
            ParseOutcome::Failed(error) => panic!("parse failed: {error:?}"),
            ParseOutcome::Complete(request) => request,
        };

        assert!(matches!(request.verb, HttpVerb::Get));
        assert_eq!(&request.path, "/path/to/../../some/sneaky/file");
        assert!(matches!(request.version, HttpVersion::OnePointOne));
        assert_eq!(request.headers.len(), 2);
        assert_eq!(request.headers.get("Content-Length"), Some("12"));
        assert_eq!(request.headers.get("Content-Type"), Some("nonsense"));
    }

    #[test]
    fn test_one_line_request() {
        let ParseOutcome::Complete(request) =
            RequestParser::new().parse(b"CONNECT / HTTP/1.0\r\n\r\n")
        else {
            panic!();
        };
        assert!(matches!(request.verb, HttpVerb::Connect));
        assert_eq!(&request.path, "/");
        assert!(matches!(request.version, HttpVersion::OnePointZero));
        assert_eq!(request.headers.len(), 0);
    }

    #[test]
    fn test_invalid_verb() {
        let ParseOutcome::Failed(ParseFailure::InvalidHttpVerb) =
            RequestParser::new().parse(b"GONT /some/path HTTP/0.9\r\n\r\n")
        else {
            panic!("GONT should have been rejected");
        };
    }

    #[test]
    fn test_verb_too_long() {
        let ParseOutcome::Failed(ParseFailure::InvalidHttpVerb) =
            RequestParser::new()
                .parse(b"CONNECTTO /some/path HTTP/1.1\r\n\r\n")
        else {
            panic!("GONT should have been rejected");
        };
    }

    #[test]
    fn test_invalid_path_byte_nul() {
        let ParseOutcome::Failed(ParseFailure::InvalidPathByte(0)) =
            RequestParser::new().parse(b"QUERY /null/\0/byte HTTP/0.9\r\n")
        else {
            panic!();
        };
    }

    #[test]
    fn test_invalid_path_byte_quote() {
        let ParseOutcome::Failed(ParseFailure::InvalidPathByte(b'"')) =
            RequestParser::new().parse(b"QUERY /quote/\" HTTP/0.9\r\n")
        else {
            panic!();
        };
    }

    #[test]
    fn test_path_too_long() {
        let ParseOutcome::Failed(ParseFailure::PathTooLong) =
            RequestParser::new().parse(
                format!(
                    "HEAD /long/ass/file/{} HTTP/1.0\r\n\r\n",
                    "a".repeat(2222)
                )
                .as_bytes(),
            )
        else {
            panic!();
        };
    }

    #[test]
    fn test_unsupported_version() {
        let ParseOutcome::Failed(ParseFailure::UnsupportedHttpVersion) =
            RequestParser::new().parse(b"POST /myfile HTTP/2.0\r\n")
        else {
            panic!();
        };
    }

    #[test]
    fn test_nonsense_in_version() {
        let ParseOutcome::Failed(ParseFailure::UnsupportedHttpVersion) =
            RequestParser::new().parse(b"GET / aaabbbccc\r\n")
        else {
            panic!();
        };
        let ParseOutcome::Failed(ParseFailure::UnsupportedHttpVersion) =
            RequestParser::new().parse(b"GET / GET\r\n")
        else {
            panic!();
        };
        let ParseOutcome::Failed(ParseFailure::UnsupportedHttpVersion) =
            RequestParser::new().parse(b"GET / \0\0\0\0\r\n")
        else {
            panic!();
        };
        let ParseOutcome::Failed(ParseFailure::UnsupportedHttpVersion) =
            RequestParser::new().parse(b"GET / HTTP/1.1 \r\n")
        else {
            panic!();
        };
    }

    #[test]
    fn test_missing_newline() {
        let ParseOutcome::Failed(ParseFailure::MissingNewline) =
            RequestParser::new()
                .parse(b"QUERY /database HTTP/0.9\rAccept: data\r\n\r\n")
        else {
            panic!();
        };
    }
}
