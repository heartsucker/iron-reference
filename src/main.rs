extern crate bincode;
extern crate csrf;
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

use csrf::{AesGcmCsrfProtection, CsrfProtection};
use handlebars_iron::{HandlebarsEngine, DirectorySource};
use iron::middleware::{AroundMiddleware, Handler, Chain, BeforeMiddleware};
use iron::prelude::*;
use iron_csrf::{CsrfConfig, CsrfProtectionMiddleware};
use secure_session::middleware::{SessionMiddleware, SessionConfig};
use secure_session::session::{SessionManager, ChaCha20Poly1305SessionManager};

use route::SessionData;

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
    let session_mgr = ChaCha20Poly1305SessionManager::from_password(password);
    let session_config = SessionConfig::default();
    let session_middleware = SessionMiddleware::new(session_mgr, session_config);

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
    let counter = PageViewCounter {};

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

// TODO DUDE LOOK HERE
// TODO DUDE LOOK HERE
// TODO DUDE LOOK HERE
// TODO DUDE LOOK HERE
// TODO DUDE LOOK HERE
// TODO DUDE LOOK HERE
// TODO this middleware needs to yank out the session data type and not the raw session
//      otherwise everyone has to write shim middleware just to make everything work
struct PageViewCounter {}

impl BeforeMiddleware for PageViewCounter {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        match request.extensions.get_mut::<SessionData>() {
            Some(session) => {
                session.get_bytes("data");
                unimplemented!() // TODO
            },
            None => unimplemented!(), // TODO
        }
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}
