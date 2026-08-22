use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub struct Reader {
    buffer: BufReader<TcpStream>,
}

impl Reader {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            buffer: BufReader::new(stream),
        }
    }

    pub fn get_request_line(&mut self) -> Result<String, std::io::Error> {
        let mut request_line = String::new();
        self.buffer.read_line(&mut request_line)?;

        Ok(request_line)
    }

    pub fn get_headers(&mut self) -> Result<Vec<String>, std::io::Error> {
        let mut headers = Vec::new();

        loop {
            let mut header = String::new();
            self.buffer.read_line(&mut header)?;

            if header == "\r\n" {
                break;
            }

            headers.push(header);
        }

        Ok(headers)
    }
}
