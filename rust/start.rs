// run rustc .\start.rs | .\start.exe

fn main(){
    let _b = "java";
    let mut a = 5;
    println!("hello world ! {{{}}}" , a);
    a = 95;
    println!("hello world ! {{{}}}" , a);
    let tup = (123 , 456 , 789);
    println!("{}" , tup.1);
    let a = [1,2,3,4,5,6,7];
    println!("{{数组{}}}" , a[5] - 1);
    println!("a[1]:{} , a[3]:{}" , a[1] , a[3]);
    let a = 128;
    fun(a);
    let mut a = -5;
    let lop = loop {
        println!("{} <= 10" , a);
        a += 1;
        if a > 10 { break a};
    };
    println!("{}" , lop);

    // 格式化输出
    println!("Hello");                 // => "Hello"
    println!("Hello, {}!", "world");   // => "Hello, world!"
    println!("The number is {}", 1);   // => "The number is 1"
    println!("{:?}", (3, 4));          // => "(3, 4)"
    println!("{value}", value=4);      // => "4"
    println!("{} {}", 1, 2);           // => "1 2"
    println!("{:04}", 42);             // => "0042" with leading zeros

    eprintln!("Error: Could not complete task");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8
    }

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{:?}", Person{name: "sunface".to_string(), age: 18});

}

fn fun(x: i32) -> i32{
    println!("javascript {}!" , { x * x + x / x - x});
    println!("{}{}10" , x ,{if x > 10 { "大于" } else {"不大于"}});
    return x;
}