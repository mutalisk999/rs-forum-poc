use std::convert::Infallible;

use askama::Template;
use hyper::http::{Request, Response};
use hyper::Body;
use routerify::ext::RequestExt;

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

// A handler for "/hello/:userId" page.
pub async fn hello_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let user_id = req.param("userId").unwrap();
    let hello_tpl = HelloTemplate {
        name: user_id.to_string(),
    };
    Ok(Response::new(Body::from(hello_tpl.render().unwrap())))
}
