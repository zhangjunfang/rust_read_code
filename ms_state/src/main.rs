extern crate actix;
extern crate actix_web;
use actix_web::{
    http, Error ,middleware, server, App, HttpRequest, HttpResponse,HttpMessage,
    AsyncResponder
};
use actix_web::middleware::session::{
    RequestSession, SessionStorage, CookieSessionBackend
};

struct State1;
struct State2;

fn main() {
    server::new(|| {
        vec![
            App::with_state(State1)
                .prefix("/app1")
                .resource("/", |r| r.f(|r| HttpResponse::Ok()))
                .boxed(),
            App::with_state(State2)
                .prefix("/app2")
                .resource("/", |r| r.f(|r| HttpResponse::Ok()))
                .boxed(),
        ]
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run()
}