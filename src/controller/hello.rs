use std::convert::Infallible;

use hyper::Body;
use hyper::http::{Request, Response};
use routerify::ext::RequestExt;
use askama::Template;


#[derive(Template)] // this will generate the code...
#[template(source = "Hello {{ name }}", ext = "txt")]
struct HelloTemplate {
    name: String,
}


// A handler for "/hello/:userId" page.
pub async fn hello_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let user_id = req.param("userId").unwrap();
    let hello_tpl = HelloTemplate { name: user_id.to_string() };
    Ok(Response::new(Body::from(hello_tpl.render().unwrap())))
}