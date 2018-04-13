
use std::sync::{Barrier,Arc};
use std::thread;
use std::sync::mpsc::{channel,TryRecvError};

fn main() {
    println!("Hello, world!");
    sync_barrier();
}


fn  sync_barrier(){
    let mut v=Vec::with_capacity(10);
    let barrier=Arc::new(Barrier::new(10));  //建立10个阻塞对象
    let (tx, rx) = channel();
    //开启十个数据，开启10个线程 一直等到第十个线程的到来
    for _ in 0..10{
        let c=  barrier.clone();
        let tx = tx.clone();
        v.push(thread::spawn(move||{
            println!("before wait");
            //一直阻塞直到第10个线程的到来
            //c.wait();
            //这里发送数据  这里发送哪个是领导线程
            tx.send(c.wait().is_leader()).unwrap();
            //所有10线程到这里集和，统一执行下面代码
            println!("after wait");
        }));
    }
    for i in v{
        let v=i.join().unwrap();
        println!("==V======={:?}===========",v);
        if rx.recv().unwrap() {
            println!("========={:?}===========",rx.recv().unwrap());
        }else {
            println!("=====else===={:?}===========",rx.recv().unwrap());
        }
    }
}