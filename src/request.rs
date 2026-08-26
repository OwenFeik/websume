use std::{collections::HashMap, path::PathBuf};

enum HttpRequestParseFailure {
    InvalidHttpVerb,
}

pub struct RequestParser(RequestParserInner);

enum HttpVerb {
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
    const MAX_LENGTH: uszie = 7;

    fn from_bytes(data: &[u8]) -> Result<Self, HttpRequestParseFailure> {
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
            _ => Err(HttpRequestParseFailure::InvalidHttpVerb),
        }
    }
}

enum HttpVersion {
    PointNine,
    OnePointZero,
    OnePointOne,
}

struct HttpRequestLine {}

struct HttpRequest {
    verb: HttpVerb,
    path: PathBuf,
    version: HttpVersion,
    headers: HashMap<String, String>,
}

enum RequestParserStep {
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
        version_text: String,
    },
    Headers {
        request: HttpRequest,
        header_name_text: String,
        header_value_text: String,
        passed_colon: bool,
        passed_space: bool,
    },
    Body(HttpRequest),
}

impl RequestParserStep {
    fn parse(self, bytes: &[u8]) -> Result<self, HttpRequestParseFailure> {
        todo!()
    }

    fn parse_byte(self, byte: u8) -> Result<self, HttpRequestParseFailure> {
        match self {
            Self::Verb { mut data } => match byte {
                b' ' => {
                    let verb = HttpVerb::from_bytes(&data)?;
                    data.clear();
                    Ok(Self::Path { verb, data })
                },
                _ => {
                    data.push(byte);
                    if data.len() > HttpVerb::MAX_LENGTH {
                        Err(HttpRequestParseFailure::InvalidHttpVerb),
                    } else {
                        Ok(Self::Verb { data })
                    }
                }
            },
            Self:Path { verb, mut data } => todo!(),
        }
    }
}

fn is_valid_path_char(byte: u8) -> bool {
    (
        (byte >= b'a' && byte <= b'z')
        || (byte >= b'A' && byte <= b'Z')
        || (byte >= b'0' && byte <= b'9')
        || byte == b'-'
        || byte == b'.'
        || byte == b'_'
        || byte == b'~'
        
    )
    
}
