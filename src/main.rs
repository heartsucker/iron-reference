extern crate bincode;
extern crate env_logger;
extern crate handlebars;
extern crate handlebars_iron;
extern crate iron;
extern crate iron_csrf;
#[cfg(test)]
extern crate iron_test;
#[macro_use]
extern crate log;
#[macro_use]
extern crate router;
extern crate secure_session;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate typemap;

mod route;

use handlebars_iron::{HandlebarsEngine, DirectorySource};
use iron::middleware::{AroundMiddleware, Handler, Chain, BeforeMiddleware};
use iron::prelude::*;
use iron_csrf::{CsrfConfig, CsrfProtectionMiddleware};
use iron_csrf::csrf::{AesGcmCsrfProtection, CsrfProtection};
use secure_session::middleware::{SessionMiddleware, SessionConfig};
use secure_session::session::{SessionManager, ChaCha20Poly1305SessionManager};
use std::collections::HashMap;

fn main() {
    env_logger::init().expect("couldn't start env logger");

    info!("Preparing server");
    let handler = get_handler();

    info!("Starting server on localhost:8080");
    Iron::new(handler).http("localhost:8080").expect("failed to start server");
}

fn get_handler() -> Box<Handler> {
    // Note: in real life, this password should come from a configuration file
    let password = b"very-very-secret";

    info!("Initializing CSRF protection");
    let protection = AesGcmCsrfProtection::from_password(password);
    let csrf_config = CsrfConfig::default();
    let csrf = CsrfProtectionMiddleware::new(protection, csrf_config);

    info!("Initializing session management");
    let session_mgr = ChaCha20Poly1305SessionManager::<Session>::from_password(password);
    let session_config = SessionConfig::default();
    let session_middleware =
        SessionMiddleware::<Session, SessionKey, ChaCha20Poly1305SessionManager<Session>>::new(session_mgr, session_config);

    info!("Initializing handlebars engine");
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("Couldn't load templates: {}", r);
    }

    info!("Building handler");
    let mut chain = Chain::new(routes());
    chain.link_before(PageViewCounter {});
    let handler = csrf.around(session_middleware.around(Box::new(chain)));
    let mut chain = Chain::new(handler);
    chain.link_after(hbse);

    // TODO chain an error catcher for things like "no route"

    Box::new(chain)
}

fn routes() -> Box<Handler> {
    Box::new(router!(
        index: get "/" => route::index,
        _static: get "/static/*" => route::_static,
    ))
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    page_views: i32,
}

impl Session {
    pub fn map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let _ = map.insert("page_views".to_string(), self.page_views.to_string());
        map
    }
}

struct SessionKey {}

impl typemap::Key for SessionKey {
    type Value = Session;
}

struct PageViewCounter {}

impl BeforeMiddleware for PageViewCounter {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        if request.url.path().len() > 0 && request.url.path()[0] != "static" {
            let session = match request.extensions.remove::<SessionKey>() {
                Some(mut session) => {
                   session.page_views += 1; 
                   session
                },
                None => {
                    Session {
                        page_views: 1,
                    }
                }
            };

            let _ = request.extensions.insert::<SessionKey>(session);
        }

        Ok(())
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}
