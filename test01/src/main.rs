extern  crate  byteorder;
use byteorder::{LittleEndian,ReadBytesExt,WriteBytesExt};
use std::vec::Vec;
fn main() {
    println!("Hello, world!");
    read_write_littleEndian();
}

#[derive(PartialOrd, PartialEq,Debug,Default)]
struct  Payload{
    kind:u8,
    value:u16,
}

fn encode(pay:&Payload)->Result<Vec<u8>>{
    let mut bytes=Vec::new();
    bytes.write_u8(pay.kind)?;
    byteorder
    Ok(bytes)
}

fn read_write_littleEndian(){

}