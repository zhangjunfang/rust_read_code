
#[macro_use]
extern crate crossbeam_channel  as channel ;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read,Write,BufReader};
use std::error::Error;
use channel::Receiver;
use channel::Sender;
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
                s.send(String::from(temp.as_ref()));
                logic(&mut t,r);
            },
            //发生异常数据处理
            Err(e) => {
                println!("===================777777======={:#?}================", e.description());
            },
        };
    }
}

fn logic(stream: &mut TcpStream,r:Receiver<String>) {
    //每次获取数据
    let mut last ="";
    while r.len()>0 {
        let mut content=r.recv().unwrap();
        if content.contains("\n") {
            //获取内容直接是以换行符结尾的情况
            if content.ends_with("\n") {
                //遍历分割的字符串
                for c in content.split("\n") {
                    if c.trim().len() != 0usize && !c.is_empty() {
                        handle(String::from(c));
                    }
                }
            } else { //不是以换行符结尾的情况，并且包含换行符
                //倒叙遍历目的：是为了获取最后不是换行符结束的数据
                let mut sign: u8 = 0u8;
                for c in content.rsplit("\n") {
                    sign = sign + 1u8;
                    if sign == 1u8 {
                        last = c;
                        continue;
                    }
                    if c.trim().len() != 0usize && !c.is_empty() {
                        handle(String::from(c));
                    }
                }
                println!("特殊情况：：：{:#?}", last);
            }
        }
    }

}

fn handle(content: String) {
    thread::spawn(move || {
        println!("thread  start  handle  ====={:#?}===========", content)
    });
}