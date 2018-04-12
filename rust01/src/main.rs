extern crate byteorder;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::OpenOptions;
use std::fs::Permissions;
use std::fs::ReadDir;
use std::io::prelude::*;
use byteorder::BigEndian;
fn main() {

    //ReadDir::next().unwrap().
    read_and_write_file_test();

}


#[warn(unused_mut)]
#[warn(unused_variables)]
//#[warn(unused_must_use)]
fn read_and_write_file_test(){
    let path="line.txt";
    //这里有问题  每次创建文件  是否可以不创建并可以追加内容
    let mut file=OpenOptions::new().write(true).read(true).create(true).append(true).open(path).unwrap();
    //let mut file= File::create(path).unwrap();
    for i  in 0..3{
        writeln!(file,"sdfsdfsdfsdf");
    }
    let readfile=File::open(path).unwrap();
    let bufread=BufReader::new(readfile);
    for line in bufread.lines() {
        println!("{}",line.unwrap());
    }
}