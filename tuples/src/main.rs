#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let tupleTest: (i8, String, bool) = (5, String::from("Test"), true);

    let black = Color(15, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
    println!("test 1: {:?}", black.0);
    println!("Tuple Test: {:?}", tupleTest)
}