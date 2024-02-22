use std::io;

fn main() {
    let x = 5;
    let mut y = x.clone();
    let y = y + 5;


    let z = 5;
    let t  = z;
    println!("z: {}, t: {}", z,t);

    println!("x: {}, y: {}", x,y)
}