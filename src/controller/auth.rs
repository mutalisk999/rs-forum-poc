use std::collections::HashMap;
use std::convert::Infallible;

use askama::Template;
use hyper::Body;
use hyper::http::{Request, Response, StatusCode};
// use routerify::ext::RequestExt;
use url::form_urlencoded;

#[derive(Template)] // this will generate the code...
#[template(source =
r###"
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <title>login</title>
</head>

<body>
  <div class="bg">
  </div>
  <div class="content">
    <input class="inputbox" type="text" name="USERNAME" placeholder="Please input Username" id="username" />
    <br />
    <input class="inputbox" type="password" name="PASSWORD" placeholder="Please input Password" id="password"/>
    <br />
    <input type="submit" value="submit" class="btn" onclick="login()"/>
  </div>
  <script src="http://libs.baidu.com/jquery/2.0.0/jquery.min.js"></script>
  <script>
    function login() {
      $.ajax({
        url: '/login',
        type: 'POST',
        async: false,
        contentType: 'application/www-form-urlencoded',
        data:{
          username:$('#username').val(),
          password:$('#password').val()
        },
        success:function(res) {
          window.location.href='/hello/world'
        },
        error:function(err) {
          console.log(err)
        }
      })
    }
  </script>
</body>

</html>
"###
, ext = "txt")]
struct LoginGetTemplate {}


// A handler for GET "/login" page.
pub async fn login_get_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let login_get_tpl = LoginGetTemplate {};
    Ok(Response::new(Body::from(login_get_tpl.render().unwrap())))
}


// A handler for POST "/login" page.
pub async fn login_post_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let req_bytes = hyper::body::to_bytes(req).await.unwrap();
    let params = form_urlencoded::parse(req_bytes.as_ref()).into_owned()
        .collect::<HashMap<String, String>>();

    let username = if let Some(name) = params.get("username") {
        name
    } else {
        return Ok(Response::builder().status(StatusCode::UNPROCESSABLE_ENTITY)
            .body("missing field username".as_bytes().into()).unwrap());
    };

    let password = if let Some(pass) = params.get("password") {
        pass
    } else {
        return Ok(Response::builder().status(StatusCode::UNPROCESSABLE_ENTITY)
            .body("missing field password".as_bytes().into()).unwrap());
    };

    //TODO
    let _ = username;
    let _ = password;

    return Ok(Response::builder().status(StatusCode::OK).body("".as_bytes().into()).unwrap());
}