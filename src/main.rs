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

mod route;

use csrf::{AesGcmCsrfProtection, CsrfProtection};
use handlebars_iron::{HandlebarsEngine, DirectorySource};
use iron::middleware::{AroundMiddleware, Handler, Chain};
use iron::prelude::*;
use iron_csrf::{CsrfConfig, CsrfProtectionMiddleware};
use secure_session::middleware::{SessionMiddleware, SessionConfig};
use secure_session::session::{SessionManager, ChaCha20Poly1305SessionManager};

fn main() {
    env_logger::init().expect("couldn't start env logger");

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
    let handler = csrf.around(session_middleware.around(Box::new(routes())));
    let mut chain = Chain::new(handler);
    chain.link_after(hbse);

    Box::new(chain)
}

fn routes() -> Box<Handler> {
    Box::new(router!(
        index: get "/" => route::index,
    ))
}
