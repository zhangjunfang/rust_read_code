#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
extern crate actix;
extern crate actix_redis;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate serde_derive;

use actix_redis::RedisSessionBackend;
use actix_web::middleware::session::{self, RequestSession};
use actix_web::{fs,middleware, server, App,
                HttpRequest, middleware::cors::Cors,
                http::{self,header, Method},HttpResponse,
                State,Form,Result
};
use actix::System;

use std::env;

mod user;
use user::info;

struct AppState {
    foo: String,
}
#[derive(Deserialize)]
pub struct MyParams {
    name: String,
}

fn index(req: &HttpRequest) -> String {
    println!("{:?}", req);
    if let Some(count) = req.session().get::<i32>("counter").unwrap_or_default() {
        println!("SESSION value: {}", count);
        req.session().set("counter", count + 1).unwrap_or_default();
        format!("在session有效周期内，第{:#?}次访问服务",count + 1)
    } else {
        req.session().set("counter", 1).unwrap_or_default();
        format!("在session有效周期内，第{:#?}次访问服务",1)
    }
}

fn index2(_req: HttpRequest<AppState>) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

fn handle_post_1(params: Form<MyParams>) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}

fn handle_post_2((state, params): (State<AppState>, Form<MyParams>), ) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain")
        .body(format!(
            "Your name is {}, and in AppState I have foo: {}",
            params.name, state.foo
        )))
}

fn handle_post_3((req, params): (HttpRequest<AppState>, Form<MyParams>), ) -> Result<HttpResponse> {
    println!("Handling POST request: {:?}", req);
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}


fn main() {
    env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");

    env_logger::init();

    let sys = System::new("basic-example");

    server::new(|| {
        vec![
            App::new()
                .middleware(middleware::Logger::default())
                .middleware(session::SessionStorage::new(
                    RedisSessionBackend::new("127.0.0.1:6379", &[0; 32])
                ))
                .resource("/", |r| r.f(index))
                .boxed(),
            App::new()
                .middleware(middleware::Logger::default())
                .handler(
                    "/fs",
                    fs::StaticFiles::new("./static/").unwrap().index_file("index.html")
                ).boxed(),
            App::new()
                .middleware(middleware::Logger::default())
                .configure(|app| {
                    Cors::for_app(app)
                        .allowed_origin("http://127.0.0.1:8080")
                        .allowed_methods(vec!["GET", "POST"])
                        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                        .allowed_header(header::CONTENT_TYPE)
                        .max_age(3600)
                        .resource("/user/info", |r| {
                            r.method(Method::POST).with(info);
                        })
                        .register()
                })
                .boxed(),
            App::with_state(AppState {
                foo: "bar".to_string(),
            }).middleware(middleware::Logger::default())
                .resource("/index", |r| {
                    r.method(http::Method::GET).with(index2);
                })
                .resource("/post1", |r| {
                    r.method(http::Method::POST).with(handle_post_1)
                })
                .resource("/post2", |r| {
                    r.method(http::Method::POST).with(handle_post_2)
                })
                .resource("/post3", |r| {
                    r.method(http::Method::POST).with(handle_post_3)
                })
                .boxed(),

        ]
    })
        .bind("0.0.0.0:8080")
        .unwrap()
        .workers(8)
        .start(); //异步运行
    let _ = sys.run();//同步阻塞

}
