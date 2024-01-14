use crate::{request::Request, response::Response};

#[derive(Clone, Debug)]
pub struct Route {
    path: String,
    method: String,
    handler: fn(Request, Response) -> Response,
}

impl Route {
    fn new(path: String, method: String, handler: fn(Request, Response) -> Response) -> Self {
        Self {
            path,
            method,
            handler,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(
        &mut self,
        path: String,
        method: String,
        handler: fn(Request, Response) -> Response,
    ) {
        self.routes.push(Route::new(path, method, handler));
    }

    pub fn handle_request(&self, request: Request) -> Response {
        let mut response = Response::new(404, "".to_string(), None);

        for route in &self.routes {
            if route.path == request.path && route.method == request.method {
                return (route.handler)(request, response);
            }
        }

        response
    }

    pub fn get(&mut self, path: String, handler: fn(Request, Response) -> Response) {
        self.add_route(path, "GET".to_string(), handler);
    }

    pub fn post(&mut self, path: String, handler: fn(Request, Response) -> Response) {
        self.add_route(path, "POST".to_string(), handler);
    }

    pub fn put(&mut self, path: String, handler: fn(Request, Response) -> Response) {
        self.add_route(path, "PUT".to_string(), handler);
    }

    pub fn delete(&mut self, path: String, handler: fn(Request, Response) -> Response) {
        self.add_route(path, "DELETE".to_string(), handler);
    }

    pub fn patch(&mut self, path: String, handler: fn(Request, Response) -> Response) {
        self.add_route(path, "PATCH".to_string(), handler);
    }

    pub fn head(&mut self, path: String, handler: fn(Request, Response) -> Response) {
        self.add_route(path, "HEAD".to_string(), handler);
    }

    pub fn options(&mut self, path: String, handler: fn(Request, Response) -> Response) {
        self.add_route(path, "OPTIONS".to_string(), handler);
    }
}
