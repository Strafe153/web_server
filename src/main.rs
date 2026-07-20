use std::{
    env,
    error::Error,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

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
            _ => Err(HttpParseError("Incorrect HTTP method.".to_string())),
        }
    }
}

#[derive(Debug)]
enum HttpVersion {
    V0_9,
    V1,
    V1_1,
}

impl TryFrom<&str> for HttpVersion {
    type Error = HttpParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "HTTP/0.9" => Ok(HttpVersion::V0_9),
            "HTTP/1.0" => Ok(HttpVersion::V1),
            "HTTP/1.1" => Ok(HttpVersion::V1_1),
            "HTTP/2" | "HTTP/3" => Err(HttpParseError(
                "Only HTTP/1.0 and HTTP/1.1 are supported.".to_string(),
            )),
            _ => Err(HttpParseError("Incorrect HTTP version".to_string())),
        }
    }
}

#[derive(Debug)]
struct HttpRequestLine {
    method: HttpMethod,
    target: String,
    version: HttpVersion,
}

#[derive(Debug)]
struct HttpParseError(String);

const CRLF: &'static str = "\r\n";

impl HttpRequestLine {
    fn parse(line: String) -> Result<HttpRequestLine, HttpParseError> {
        if !line.ends_with(CRLF) {
            return Err(HttpParseError(
                "Request line does NOT end with CRLF.".to_string(),
            ));
        }

        let line = line.trim_end_matches(CRLF);
        let mut split = line.split(' ');
        let parts: Vec<&str> = split.clone().collect();

        if parts.iter().count() != 3 {
            return Err(HttpParseError(
                "Requst line MUST consist of 3 parts.".to_string(),
            ));
        }

        if split.any(|x| x == "") {
            return Err(HttpParseError(
                "Request line part CANNOT be empty.".to_string(),
            ));
        }

        let method = HttpMethod::try_from(parts[0])?;
        let target = parts[1].to_string();
        let version = HttpVersion::try_from(parts[2])?;

        let request_line = HttpRequestLine {
            method,
            target,
            version,
        };

        return Ok(request_line);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut port = 1717;

    if args.len() > 1 {
        // gracefully handle error
        port = args[1].parse::<u16>().unwrap();
    }

    // if error - port is occupied, handle that
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("Listening");

    for stream in listener.incoming() {
        let temp = stream.unwrap();
        // handle error
        let request_line = read_request_line(temp)?;
        let t = HttpRequestLine::parse(request_line);
        println!("{:#?}", t);
    }

    Ok(())
}

fn read_request_line(stream: TcpStream) -> Result<String, std::io::Error> {
    let mut reader = BufReader::new(&stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    Ok(request_line)
}
