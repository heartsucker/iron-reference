use handlebars_iron::Template;
use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use std::fs::File;

use super::SessionKey;


pub fn index(request: &mut Request) -> IronResult<Response> {
    let session = request.extensions.get::<SessionKey>().unwrap();
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", session.map()))
        .set_mut(status::Ok);
    Ok(resp)
}

pub fn _static(request: &mut Request) -> IronResult<Response> {
    let path: String = request.url
        .path()
        .iter()
        .filter(|s| !s.is_empty() && **s != "..")
        .map(|s| format!("/{}", s))
        .collect();

    match File::open(format!(".{}", path)) {
        Ok(f) => {
            let mut response = Response::with(f);
            response.status = Some(status::Ok);
            if path.ends_with(".css") {
                response.headers.set(ContentType("text/css".parse().unwrap()));
            }

            Ok(response)
        },
        Err(e) => {
            info!("Error loading static resource {}: {}", path, e);
            Ok(Response::with((status::NotFound, "Not Found")))
        },
    }
}
