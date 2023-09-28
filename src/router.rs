use std::convert::Infallible;

use hyper::http::{Request, Response, StatusCode};
use hyper::Body;
use log::{debug, error};
use routerify::ext::RequestExt;
use routerify::{Middleware, RequestInfo, Router};

use crate::controller::auth::{login_get_handler, login_post_handler};
use crate::controller::hello::hello_handler;

// Define an app state to share it across the route handlers and middlewares.
struct State(u64);

// A middleware which logs an http request.
async fn logger(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    debug!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

// Define an error handler function which will accept the `routerify::Error`
// and the request information and generates an appropriate response.
async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

// Create a `Router<Body, Infallible>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
pub fn register_router() -> Router<Body, Infallible> {
    // Create a router and specify the logger middleware and the handlers.
    // Here, "Middleware::pre" means we're adding a pre middleware which will be executed
    // before any route handlers.
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(State(100))
        .middleware(Middleware::pre(logger))
        .get("/hello/:userId", hello_handler)
        .get("/login", login_get_handler)
        .post("/login", login_post_handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}
