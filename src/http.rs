use std::fmt;

const CRLF: &'static str = "\r\n";
const HEADER_VALUE_MAX_LENGTH: usize = 8192;

#[derive(Debug)]
pub struct HttpParseError(String);

impl HttpParseError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for HttpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for HttpParseError {}

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

impl TryFrom<&str> for HttpMethod {
    type Error = HttpParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "PATCH" => Ok(HttpMethod::PATCH),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err(HttpParseError::new("Invalid HTTP method.")),
        }
    }
}

#[derive(Debug)]
pub struct HttpRequestLine {
    method: HttpMethod,
    target: String,
}

impl TryFrom<String> for HttpRequestLine {
    type Error = HttpParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.ends_with(CRLF) {
            return Err(HttpParseError::new("Request line does NOT end with CRLF."));
        }

        let value: &str = value.trim_end_matches(CRLF);
        let mut split = value.split(' ');
        let parts: Vec<&str> = split.clone().collect();

        if parts.iter().count() != 3 {
            return Err(HttpParseError::new("Requst line MUST consist of 3 parts."));
        }

        if split.any(|p| p == "") {
            return Err(HttpParseError::new("Request line part CANNOT be empty."));
        }

        let method = HttpMethod::try_from(parts[0])?;
        let target = parts[1].to_string();

        let http_version = "HTTP/1.1";
        if parts[2] != http_version {
            return Err(HttpParseError::new("Only HTTP/1.1 is supported."));
        }

        let request_line = HttpRequestLine { method, target };

        return Ok(request_line);
    }
}

#[derive(Debug)]
pub struct HttpHeader {
    name: String,
    value: String,
}

impl TryFrom<String> for HttpHeader {
    type Error = HttpParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.contains(':') {
            return Err(HttpParseError::new("No colon present"));
        }

        let value = value.trim_end_matches(CRLF);

        let split = value.split(':');
        let parts: Vec<&str> = split.clone().collect();

        // this also handles line folding - should return 400 saying that obsolete line folding is unacceptable,
        // since it's been deprecated everywhere other than in message/http
        if parts.iter().count() != 2 {
            return Err(HttpParseError::new(
                "Header must consist of two parts: name and value",
            ));
        }

        if parts[0].ends_with(" ") {
            return Err(HttpParseError::new(
                "Header name cannot contain trailing spaces",
            ));
        }

        if parts[1] == "" {
            return Err(HttpParseError::new("Header value cannot be empty"));
        }

        if parts[1].len() > HEADER_VALUE_MAX_LENGTH {
            return Err(HttpParseError(format!(
                "Header value cannot be longer than {}",
                HEADER_VALUE_MAX_LENGTH
            )));
        }

        let name = parts[0].to_string();
        let value = parts[1].trim().to_string();

        let header = HttpHeader { name, value };

        Ok(header)
    }
}
