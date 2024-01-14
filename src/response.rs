use std::net::TcpStream;
use std::io::{prelude::*, BufWriter};

#[derive(Clone, Debug)]
pub struct Response {
  pub status: u32,
  pub body: String,
  pub content_type: Option<String>,
}

impl Response {
  pub fn new(status: u32, body: String, content_type: Option<String>) -> Self {
    Self {
      status,
      body,
      content_type,
    }
  }

  pub fn status(&self, status: u32) -> Self {
    Self {
      status,
      body: "".to_string(),
      content_type: None,
    }
  }

  pub fn json(&self, status: u32, body: String) -> Self {
    Self {
      status,
      body,
      content_type: Some("application/json".to_string()),
    }
  }

  pub fn html(&self, status: u32, body: String) -> Self {
    Self {
      status,
      body,
      content_type: Some("text/html".to_string()),
    }
  }

  pub fn text(&self, status: u32, body: String) -> Self {
    Self {
      status,
      body,
      content_type: Some("text/plain".to_string()),
    }
  }

  pub fn redirect(&self, status: u32, location: String) -> Self {
    Self {
      status,
      body: "".to_string(),
      content_type: Some(format!("text/html\r\nLocation: {}", location)),
    }
  }

  pub fn send(&self, stream: &mut TcpStream) {
    let mut writer = BufWriter::new(stream);

    write!(writer, "HTTP/1.1 {}\r\n", self.status).unwrap();
    write!(writer, "Content-Length: {}\r\n", self.body.len()).unwrap();

    if let Some(content_type) = &self.content_type {
      write!(writer, "Content-Type: {}\r\n", content_type).unwrap();
    }

    write!(writer, "\r\n").unwrap();

    writer.write_all(self.body.as_bytes()).unwrap();
    writer.flush().unwrap();
  }
}