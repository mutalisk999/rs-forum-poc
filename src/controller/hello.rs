use std::convert::Infallible;

use hyper::Body;
use hyper::http::{Request, Response};
use routerify::ext::RequestExt;
use askama::Template;


#[derive(Template)] // this will generate the code...
#[template(source =
r###"
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <title>hello</title>
</head>

<body>
  <h1> Hello {{ name }} </h1>
</body>

</html>
"###, ext = "txt")]
struct HelloTemplate {
    name: String,
}


// A handler for "/hello/:userId" page.
pub async fn hello_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let user_id = req.param("userId").unwrap();
    let hello_tpl = HelloTemplate { name: user_id.to_string() };
    Ok(Response::new(Body::from(hello_tpl.render().unwrap())))
}