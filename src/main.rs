mod app;
mod request;
mod response;
mod router;

use app::App;
use router::Router;

fn main() {
    let mut app = App::new();

    // create a new router
    let mut router = Router::new();

    // add a route
    router.get("/".to_string(), |_req, res| {
        res.html(200, "<h1>Hello, World!</h1>".to_string())
    });

    // add a route with a parameter
    router.get("/user/:id".to_string(), |req, res| {
        let id = req.params[0].1.clone();

        res.html(200, format!("<h1>User ID: {}</h1>", id))
    });

    // add a route with a json response
    router.post("/json".to_string(), |_req, res| {
        res.json(200, r#"{"message": "Hello, World!"}"#.to_string())
    });

    // add the router to the app
    app.add_router(router);

    // set the not found handler
    app.use_not_found_handler(|_req, res| {
        res.html(404, "<h1>Not Found</h1>".to_string())
    });

    // start the server
    app.listen("127.0.0.1", 3000, Some(|| {
        println!("Started server on port {}", 3000);
    }));
}