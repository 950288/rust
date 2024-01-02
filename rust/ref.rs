macro_rules! add {
    ($a:expr, $b:expr) => {
        (|| {
            println!("macro");
            $a + $b
        })()
        
    };
}

fn main() {
    let ref a = 10;
    println!("{}", add!(a, 15));
}
