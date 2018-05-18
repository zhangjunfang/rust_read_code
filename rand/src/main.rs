



//#[macro_use]
//extern crate error_chain;
//extern crate byteorder;
extern crate rand;



use rand::Rng;
use rand::distributions::{Range, IndependentSample};
use rand::distributions::Normal;

fn rand_01() {
    // Each thread has an automatically-initialised random number generator:
    let mut rng = rand::thread_rng();

    // Integers are uniformly distributed over the type's whole range:
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());

    // Floating point numbers are uniformly distributed in the half-open range [0, 1)
    println!("Random float: {}", rng.gen::<f64>());
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));

    let mut rng = rand::thread_rng();
    let die = Range::new(1, 7);

    loop {
        let throw = die.ind_sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    let mut rng = rand::thread_rng();

    // mean 2, standard deviation 3:
    let normal = Normal::new(2.0, 3.0);
    let v = normal.ind_sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);

    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);

    let rand_string: String = rand::thread_rng().gen_ascii_chars().take(30).collect();
    println!("{}", rand_string);


    const CHARSET: &[u8] =  b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    let mut rng = rand::thread_rng();
    let password: Option<String> = (0..30)
        .map(|_| Some(*rng.choose(CHARSET)? as char))
        .collect();

    println!("{:?}", password);


}

use rand::{ Rand};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Rand for Point {
    fn rand<R: Rng>(rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

//==========================22==========================
fn main() {
    println!("Hello, world!");
    //run();
    rand_01();
}
