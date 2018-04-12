
use  std::thread;
use  std::sync::mpsc::channel;
use   std::time::Duration;
fn main() {
    //thread_result();
    //thread_mpsc();
    simple_thread();
    thread::yield_now();
    //不释放资源情况下 休眠一段时间
    thread::sleep_ms(2000);
    println!("Hello, world!");
}
//获取线程执行结果
fn   thread_result(){
    let handle=thread::spawn(move||{

        " hello   zhangboyu "
    });
    let value=handle.join().expect("The receiver data has panicked");
    println!("value==========={}",value);
}


///多生产者 单消费者
///
fn  thread_mpsc(){
   let (sender,receiver)= channel();
   let sender_handle= thread::spawn(move||{
        sender.send(" hello  zhangboyu ").expect("发送数据出错了！！！");

    });
    let receiver_handle= thread::spawn(move||{
        let v=receiver.recv().expect("接收数据出错了！！！");
        println!("=====value ===={:?}======",v);
    });
    sender_handle.join().expect("The sender thread has panicked");
    receiver_handle.join().expect("The receiver thread has panicked");
}

fn  simple_thread(){
    //父线程向子线程之间传递数据
    //thread_local!(name::0001);
//    let name=thread::current().name().unwrap();
//    println!("==name====={:?}==========",name);
    //设置stack初始大小  实际上也许比这个数值大
    let handle=thread::Builder::new().
        name("zhangboyu".to_owned()).stack_size(1024*1024).spawn(move ||{
        //获取当前线程名称
        let t = thread::current();
        println!("=thread=name====={:?}==========",t.name());
        //获取线程id
        let id=t.id();
        println!("=thread=id====={:?}==========",t.id());
        println!("=thread=id====={:?}==========",id.to_owned());
        //
        thread::sleep(Duration::from_millis(10));
        println!("======={:?}==========",12);
    });

    handle.expect("sdfsfsdfsd").join();
}