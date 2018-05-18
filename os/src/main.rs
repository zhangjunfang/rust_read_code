use std::collections::VecDeque;
fn main() {
    println!("Hello, world!");
    
    {
        use std::collections::btree_map::BTreeMap;
         let mut count = BTreeMap::new();
         let message = "she sells sea shells by the sea shore";
        
         for c in message.chars() {
             *count.entry(c).or_insert(0) += 1;
         }
        
         assert_eq!(count.get(&'s'), Some(&8));
        
         println!("Number of occurrences of each character");
         for (char, count) in &count {
             println!("{}: {}", char, count);
         }
    }
    
    
    
    {
        let vec = vec![1, 2, 3, 4];
        for x in vec.iter().rev() {
           println!("vec contained {}", x);
        }
    }
    
    

    {
         let mut vec1 = vec![1, 2, 3, 4];
         let vec2 = vec![10, 20, 30, 40];
         vec1.extend(vec2);
        println!("Hello, world!   {:?}",vec1.len());
        //let l=vec1.into_iter().collect();
    }
    
    
    {
        let mut vec = vec![1, 2, 3, 4];
        for x in vec.iter_mut() {
            
            *x += 1;
        }
    }

    {
        let vec = vec![1, 2, 3, 4];
        for x in vec.iter() {
            let x=*x;
            println!("vec contained {}", x);
        }
    }


}
