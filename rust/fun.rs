fn main() {
    println!("x plus y equals {}",another_function(5, 6))
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("Hello, runoob!");
    println!("x is {}", x);
    println!("y is {}", y);
    return x + y;
}