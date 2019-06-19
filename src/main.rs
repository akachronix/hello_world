use std::vec::Vec;

fn main() {
    
    let mut vec = Vec::new();
    for i in 0..10 {
        vec.push(i);
    }

    for x in &vec {
        print!("{} ", x);
    }

    print!("\n");
}
