use std::net::TcpListener;
use std::net::SocketAddr;
use std::net::TcpStream;

use crate::{request::Request, response::Response, router::Router};

pub struct App {
    routers: Vec<Router>,
    not_found_handler: fn(Request, Response) -> Response,
    middlewares: Vec<fn(Request, Response) -> (Request, Response)>,
}

impl App {
    pub fn new() -> Self {
        Self {
            routers: Vec::new(),
            not_found_handler: |_req, res| res,
            middlewares: Vec::new(),
        }
    }

    pub fn use_middleware(&mut self, middleware: fn(Request, Response) -> (Request, Response)) {
        self.middlewares.push(middleware);
    }

    pub fn use_not_found_handler(&mut self, handler: fn(Request, Response) -> Response) {
        self.not_found_handler = handler;
    }

    pub fn add_router(&mut self, router: Router) {
        self.routers.push(router);
    }

    pub fn listen(&self, host: &str, port: u32, handler: Option<fn() -> ()>) {
        let address: SocketAddr = format!("{}:{}", host, port)
            .parse()
            .expect("Failed to parse address");
        let listener = TcpListener::bind(address);

        match listener {
            Ok(listener) => {
                println!("Listening on: {}", listener.local_addr().unwrap());

                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            println!("New connection: {}", stream.peer_addr().unwrap());

                            self.process_request(stream, handler.as_ref().cloned());
                        }
                        Err(e) => {
                            println!("Failed to establish a connection: {}", e)
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to bind to port 3000: {}", e)
            }
        }
    }

    pub fn process_request(&self, stream: TcpStream, handler: Option<fn() -> ()>) {
        let app_routers = self.routers.clone();
        let app_not_found_handler = self.not_found_handler;
        let app_middlewares = self.middlewares.clone();

        std::thread::spawn(move || {
            if let Some(mut request) =
                Request::parse(stream.try_clone().unwrap())
            {
                let mut response = Response::new(404, "".to_string(), None);

                for middleware in &app_middlewares {
                    let (req, res) = middleware(request.clone(), response);
                    request = req;
                    response = res;
                }

                for router in &app_routers {
                    response = router.handle_request(request.clone());

                    if response.status != 404 {
                        break;
                    }
                }

                let mut stream = request.stream.lock().unwrap();

                if response.status == 404 {
                    response = app_not_found_handler(request.clone(), response);
                }

                response.send(&mut stream);
            } else {
                println!("Failed to parse request");
            }
        });

        if let Some(handler) = handler {
            handler();
        }
    }
}
