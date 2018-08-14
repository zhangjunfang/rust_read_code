extern crate thread_pool;
extern crate crossbeam_channel  as channel ;
extern crate  num_cpus as cpu ;
use thread_pool::ThreadPool;
use std::net::{TcpListener};
use std::thread;
use std::io::{Read};
use std::error::Error;
use channel::Receiver;
use thread_pool::Task;
use std::vec::Vec;
struct  Message{
    ms:String,
}
//执行线程池中的任务
impl Task for Message{
    fn run(self){
        println!("thread  start  handle  ====={:#?}===========", self.ms )
    }
}

fn main() {
    let tcplistener = TcpListener::bind("0.0.0.0:9090").unwrap();
    let ttl = tcplistener.set_ttl(200u32);
    match ttl {
        Ok(_t) => {}
        Err(e) => {
            println!("1111111111111111===={:#?}", e.description());
        }
    }
    let (s,r)=channel::bounded::<String>(4096*1024);
    let rr=r.clone();
    let child = thread::spawn(move || {
        logic(rr);
    });


    for stream in tcplistener.incoming() {
        match stream {
            //正常获取数据
            Ok(t) => {
                let mut t = t;
                let ttl = t.set_ttl(100u32);
                match ttl {
                    Ok(_t) => {}
                    Err(e) => {
                        println!("{:#?}", e.description());
                    }
                }
                let mut buffer = [0u8; 1024];
                let us = t.read(&mut buffer).unwrap();
                let mut temp = String::from_utf8_lossy(&buffer[..us]);
                //每次获得到的字符串消息追加到队列中
                s.send(String::from(temp));
            },
            //发生异常数据处理
            Err(e) => {
                println!("===================777777======={:#?}================", e.description());
            },
        };
    }
    child.join();
}

fn logic(r:Receiver<String>) {
    //开启线程池处理逻辑
    let (s,p)= ThreadPool::fixed_size(cpu::get());
    //每次获取数据
    let mut last ="";
    while r.len()>0 {
        let  mut content = r.recv().unwrap();
        content = last.to_owned() + content.trim();
        if  content.contains("\n") {
            let mut b :bool=false;
            if ! content.ends_with("\n") {
                b=true;
            }
            let v: Vec<String>=content.split("\n").collect();
            let mut i: usize=0usize;
            let len=v.len();
            for c in v{
                i=i+1usize;
                let m=c.trim();
                if b && i==len {
                    last = &*c;
                    b=false;
                    continue;
                }
                if ! m.is_empty(){
                    //s.send(Message{ms:String::from(m)});
                    s.send(Message{ms:c});
                }
            }
        }else{
            last=&content;
        }
        println!("不完整数据：：：{:#?}",last);
    }
   p.await_termination();
}

