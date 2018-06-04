extern  crate   rand;

use rand::{thread_rng, Rng};
fn main(){
   let mut m= thread_rng();
   for _ in 0..10{
      println!("{}",m.gen::<i32>());
   }

}
