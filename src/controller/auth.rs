use std::collections::HashMap;
use std::convert::Infallible;
use log::{error, warn, info, debug};

use askama::Template;
use hyper::Body;
use hyper::http::{Request, Response, StatusCode};
// use routerify::ext::RequestExt;
use url::form_urlencoded;
use crate::model::t_user::query_t_user_by_name;
use crate::utils::jwt::{create_jwt, Role};

#[derive(Template)] // this will generate the code...
#[template(path = "login.html")]
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

    let res = query_t_user_by_name(username).await;
    if res.is_err() {
        error!("query_t_user_by_name fail: {}", res.unwrap_err().to_string());
        return Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("query_t_user_by_name fail".as_bytes().into()).unwrap());
    }
    let res = res.unwrap();

    // username not found
    if res.is_none() {
        debug!("username {} not found", &username);
        return Ok(Response::builder().status(StatusCode::NOT_FOUND)
            .body("username not found".as_bytes().into()).unwrap());
    } else {
        let res = res.unwrap();
        // invalid password
        if res.pass.unwrap() != *password {
            debug!("invalid password for username {}", &username);
            return Ok(Response::builder().status(StatusCode::FORBIDDEN)
                .body("invalid password for username".as_bytes().into()).unwrap());
        }

        let id = res.id.unwrap().to_string();
        let res = create_jwt(&id, &Role::Admin, 3600).await;
        // create jwt fail
        return if res.is_err() {
            Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("create jwt fail".as_bytes().into()).unwrap())
        } else {
            Ok(Response::builder().status(StatusCode::OK)
                .header("Authorization", res.unwrap())
                .body("".as_bytes().into()).unwrap())
        }
    }
}