
use std::path::{self,Path,PathBuf};

fn main() {
    println!("Hello, world!");
    update_file_paht();
    file_path();
}

fn file_path(){
    let p=Path::new("/home/roo/code/rust/file01");
    //获取当前路径的父路径
    p.parent();
    //当前目录或者文件的名称
    let  file_stem=p.file_stem().unwrap();
    println!("------{:?}--------",file_stem);
    //获取文件扩展名
    let ext=p.extension();
    //文件是否存在
    let exits=p.exists();
    //
    let file_name=p.file_name().unwrap();
    //
}
fn  update_file_paht(){
    let mut p=PathBuf::from("/home/roo/code/rust/file01");
    p.push("/001");
    p.push("/002");
    p.set_extension("dll");
    println!("======{:?}========",p);


}