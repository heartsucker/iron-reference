use handlebars_iron::Template;
use iron::prelude::*;
use iron::status;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", String::new()))
        .set_mut(status::Ok);
    Ok(resp)
}
