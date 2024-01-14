use std::net::TcpStream;
use std::io::{prelude::*, BufReader};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Request {
  pub path: String,
  pub method: String,
  pub body: Option<String>,
  pub query: Option<String>,
  pub params: Vec<(String, String)>,
  pub stream: Arc<Mutex<TcpStream>>,
}

impl Request {
  pub fn new(path: String, method: String, body: Option<String>, query: Option<String>, params: Vec<(String, String)>, stream: Arc<Mutex<TcpStream>>) -> Self {
    Self {
      path,
      method,
      body,
      query,
      params,
      stream,
    }
  }

  pub fn parse_query(query: &str) -> Option<String> {
    if let Some(index) = query.find("?") {
      let (_, query) = query.split_at(index + 1);
      return Some(query.to_string());
    }

    None
  }

  pub fn parse_params(path: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();

    for (index, part) in path.split("/").enumerate() {
      if part.starts_with(":") {
        params.push((part[1..].to_string(), index.to_string()));
      }
    }

    params
  }

  pub fn parse_json(body: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(body)
  }

  pub fn parse(stream: TcpStream) -> Option<Self> {
    let mut buffer = BufReader::new(&stream);
    let mut request = String::new();

    match buffer.read_line(&mut request) {
      Ok(_) => {
        let mut content_length: usize = 0;

        loop {
          let mut header = String::new();

          match buffer.read_line(&mut header) {
            Ok(_) => {
              if header.trim().is_empty() {
                break;
              }

              if header.to_lowercase().starts_with("content-length") {
                if let Some(length_str) = header.split(":").nth(1) {
                  if let Ok(length) = length_str.trim().parse::<usize>() {
                    content_length = length;
                  }
                }
              }
            }
            Err(e) => {
              eprintln!("Failed to read header: {}", e);
              break;
            }
          }
        }

        if content_length > 0 {
          let mut body = vec![0; content_length];

          match buffer.read_exact(&mut body) {
            Ok(_) => {
              if let Ok(payload) = String::from_utf8(body) {
                let json = match Self::parse_json(&payload) {
                  Ok(json) => json.to_string(),
                  Err(e) => format!("Failed to parse JSON: {}", e),
                };

                return Some(Self::new(
                  request.split_whitespace().nth(1).unwrap().to_string(),
                  request.split_whitespace().next().unwrap().to_string(),
                  Some(json),
                  Self::parse_query(&request),
                  Self::parse_params(&request),
                  Arc::new(Mutex::new(stream)),
                ));
              }
            }
            Err(e) => {
              eprintln!("Failed to read body: {}", e);
            }
          }
        }
      }
      Err(e) => {
        eprintln!("Failed to read from stream: {}", e);
      }
    }

    None
  }

  pub fn params(&self) -> Vec<(String, String)> {
    self.params.clone()
  }

  pub fn param(&self, name: &str) -> Option<String> {
    for (param_name, param_value) in &self.params {
      if param_name == name {
        return Some(param_value.to_string());
      }
    }

    None
  }

  pub fn query(&self) -> Option<String> {
    self.query.clone()
  }

  pub fn query_param(&self, name: &str) -> Option<String> {
    if let Some(query) = &self.query {
      let query_params: Vec<&str> = query.split("&").collect();

      for query_param in query_params {
        let query_param_parts: Vec<&str> = query_param.split("=").collect();

        if query_param_parts.len() == 2 {
          let query_param_name = query_param_parts[0];
          let query_param_value = query_param_parts[1];

          if query_param_name == name {
            return Some(query_param_value.to_string());
          }
        }
      }
    }

    None
  }

  pub fn body(&self) -> Option<String> {
    self.body.clone()
  }

  pub fn json<T: serde::de::DeserializeOwned>(&self) -> Option<T> {
    if let Some(body) = &self.body {
      return match serde_json::from_str(body) {
        Ok(json) => Some(json),
        Err(_) => None,
      };
    }

    None
  }
}