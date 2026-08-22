mod http;
mod reader;

use http::{HttpHeader, HttpParseError, HttpRequestLine};
use reader::Reader;
use std::{env, net::TcpListener};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = match env::args().nth(1) {
        Some(a) => match a.parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                println!("Invalid port");
                std::process::exit(1);
            }
        },
        None => 1717,
    };

    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(listener) => {
            println!("Listening...");

            for stream_result in listener.incoming() {
                match stream_result {
                    Ok(stream) => {
                        let mut reader = Reader::new(stream);

                        // instead of using ? handle errors here, since right now it's main (which should be refactored)
                        // but it's the last chance to gracefully handle them
                        let request_line = reader.get_request_line()?;
                        let request_line_result = HttpRequestLine::try_from(request_line);
                        println!("{:#?}", request_line_result);

                        let headers = reader.get_headers()?;
                        let headers: Result<Vec<HttpHeader>, HttpParseError> =
                            headers.into_iter().map(HttpHeader::try_from).collect();

                        if let Err(e) = headers {
                            println!("{}", e);
                            continue;
                        }

                        println!("{:#?}", headers);
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        Err(_) => println!("Failed to bind to TCP port {}.", port),
    }

    Ok(())
}
