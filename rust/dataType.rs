fn main() {
    //  https://www.runoob.com/rust/rust-data-types.html
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // x += 1.0; cannot assign twice to immutable variable
    println!("a is {}", x);

    // 元组是用一对 ( ) 包括的一组数据，可以包含不同种类的数据：
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is {:?}", tup);

    // 数组用一对 [ ] 包括的同类型数据。
    let a = [1, 2, 3, 4, 5];
    // a[0] = 123; 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确

    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 is {}", s1); // 错误：s1 的所有权已经转移到 s2
    println!("s2 is {}", s2);
    let s3 = s2.clone() + " world";
    println!("{}", s3);
}