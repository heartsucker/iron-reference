use handlebars_iron::Template;
use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use std::fs::File;
use typemap;

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub page_view: i32,
}

impl typemap::Key for SessionData {
    type Value = SessionData;
}


pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", String::new()))
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
