

use std::path;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry};
//文件操作相关的示例

fn main() {

    let p= Path::new("/home/roo/code/rust/file01");
    visit_dirs(p);
    let  perfix="file";
    metaDate_file(perfix);
}

fn visit_dirs(dir: &Path)->io::Result<()>  {
         if dir.is_dir() {
             for entry in fs::read_dir(dir)? {
                 let entry = entry?;
                 let path = entry.path();
                 if path.is_dir() {
                     visit_dir(&path)?;
                     println!("-----dir name --------{:?}--------",path);
                 } else {
                     println!("-----file name --------{:?}--------",path.file_name().unwrap());
                 }
             }
         }
    Ok(())
     }

fn  metaDate_file(px:&str){
    //字符串连接
    let  file=px.to_owned();
    //创建多层目录：
    fs::create_dir_all(file);
    //创建一层目录
    //fs::create_dir(file);
    //创建文件：  如果存在直接追加 同时具备读写创建追加权限
    let file=px.to_owned()+"/aa.txt";
    let f= fs::OpenOptions::new()
                .read(true).write(true).create(true)
                .append(true).open(file).unwrap();
    //获取文件的元数据
    let meta=f.metadata().unwrap();
    //具体元数据参见如下：
    meta.accessed();
    meta.created();
    meta.file_type();
    meta.is_dir();
    meta.is_file();
    meta.len();
    meta.permissions();
    meta.modified();
}
