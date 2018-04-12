//extern crate byteorder;

use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fs::File;
use std::fs::File;

//use byteorder::
fn main() {
    read_and_write_file_test();

}
#[derive(Default,PartialOrd, PartialEq,Debug)]
pub struct  Payload{
    pub kind:u8,
    pub value:u16,
}
//fn read_and_write_integers_little_endian()->Result<()>{
//
//    //let mut pay=Payload::default();
//
//
//
//    Ok(())
//}

//fn encode(pay : &payload)->Result<Vec<u8>>{
//    let mut bytes = vec![];
//    bytes.wr
//}























#[warn(unused_mut)]
#[warn(unused_variables)]
//#[warn(unused_must_use)]
fn read_and_write_file_test(){
    let path="line.txt";
    //这里有问题  每次创建文件  是否可以不创建并可以追加内容
    let mut file= File::create(path).unwrap();
    for i  in 0..3{
        writeln!(file,"sdfsdfsdfsdf");
    }
    let readfile=File::open(path).unwrap();
    let bufread=BufReader::new(readfile);
    for line in bufread.lines() {
        println!("{}",line.unwrap());
    }
}

