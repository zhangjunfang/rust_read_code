extern crate actix;
extern crate actix_web;
extern crate env_logger as log ;
extern crate  num_cpus as cpu;
extern crate futures;
extern crate bytes;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;
use actix::{System};
use std::env;
use std::cell::Cell;
use actix_web::{
    http, Error ,middleware, server, App, HttpRequest, HttpResponse,HttpMessage,
    AsyncResponder,pred
};
use actix_web::middleware::session::{
    RequestSession, SessionStorage, CookieSessionBackend
};
use bytes::BytesMut;
use futures::{Future, Stream};
use json::JsonValue;
#[derive(Debug, Serialize, Deserialize)]
struct  Person{
    id : u32,
    name : String ,
    age : u8 ,
}


//资源被访问的次数
struct AppState {
    counter: Cell<usize>,
}
struct AppState2 {
    counter: Cell<usize>,
}
fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "张伯雨 first rust web !"
}
fn index_html(_req: &HttpRequest<AppState>)->String {

    String::from("<h2>zhang bo yu  </h2>")
}

fn state(req:&HttpRequest<AppState>)-> HttpResponse {
    println!("{:?}", req);
    req.state().counter.set(req.state().counter.get() + 1);

    HttpResponse::Ok()
        .body(format!("Num of requests: {}", req.state().counter.get()))
}

fn json(req :&HttpRequest<AppState2>)-> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()  // convert all errors into `Error`
        .and_then(|val: Person| {
            println!("model: {:?}", val);
            Ok(HttpResponse::Ok().json(val))  // <- send response
        })
        .responder()
}

fn session(req : &HttpRequest<AppState>)->String{
      if  let Some(id)=req.session().get::<u32>("id").unwrap_or_default(){
          req.session().set("id",id+1u32);
          format!("当前应用被访问的第{:#?}次",id+1u32)
      }else{
          req.session().set("id",1u32);
          format!("当前应用被访问的第{:#?}次",1u32)
      }
}

//集成全局状态state  过程出错了
fn main() {

    //设置系统参数，提供给日志中间件使用
    env::set_var("RUST_LOG", "actix_web=info");
    log::init(); //初始化日志相关的参数
    let sys = System::new("hello-world");//创建Actix系统
   // let mut v=Vec::with_capacity(16usize );
    server::new(|| { //使用应用工厂创建新的http服务器
        vec![
             App::with_state(AppState{counter: Cell::new(0)})
                 //new()
                 .prefix("/mm")  //添加单一应用的统一前缀
                 //限制访问  必须是post提交  否则找不到资源
                 .filter(pred::Method(http::Method::GET)) //前置过滤器
                 .middleware(middleware::Logger::default()) // 日志中间件
                 .resource("hh",|r|{r.f(|_:&HttpRequest<AppState>|{"hello  visitor !!!!"})})
                 .boxed(),//多状态 必须使用这个方法
             App::with_state(AppState{counter: Cell::new(0)})
                 //new()
                 .prefix("/user")  //添加单一应用的统一前缀
                 .middleware(middleware::Logger::default())// 日志中间件
                 .middleware(middleware::DefaultHeaders::new().header("X-Version", "0.2"))  //添加请求头
                 .middleware(SessionStorage::new(//创建session中间件
                                                 CookieSessionBackend::signed(&[0; 32])//后端存储session中间件
                                                     .secure(false)
                 ))//添加session中间件
                 .resource("/index.html", |r| r.f(|_| "Hello 张伯雨 !"))//直接资源路由和资源处理逻辑
                 .resource("", |r| r.f(index))//路由空字符串对应方法处理handler为index函数
                 .resource("/",|r|{ r.f(index_html)})//路由字符串“/”对应方法处理handler为index_html函数
                 //.resource("/json",|r|{ r.f(json)})//路由空字符对应方法处理handler为index函数
                 .resource("/session",|r|{ r.f(session)})//路由字符创/session对应方法处理handler为session函数);
                 .boxed(),//多状态 必须使用这个方法
             App::with_state(AppState{counter: Cell::new(0)}) // <- create app with state
                 // enable logger
                 .prefix("/state")  //添加单一应用的统一前缀
                 .middleware(middleware::Logger::default())
                 // register simple handler, handle all methods
                 .resource("/", |r| r.f(state))
                 .boxed(),//多状态 必须使用这个方法

             App::with_state(AppState2{counter: Cell::new(0)}) // <- create app with state
                 .prefix("/json")  //添加单一应用的统一前缀
                 .middleware(middleware::Logger::default())// enable logger
                 .resource("/", |r| r.method(http::Method::GET).f(json))
                 .boxed(),//多状态 必须使用这个方法
        ]})
        .workers(cpu::get())
        .bind("127.0.0.1:8080")//绑定应用监听地址为127.0.0.1:8080地址
        .unwrap()//异常处理
        .start();//开启监听
    //System::current().stop(); //停止当前运行的actix系统
    sys.run();//运行actix系统，此方法启动所有异步进程
}