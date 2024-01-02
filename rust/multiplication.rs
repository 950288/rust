fn main(){
    let mut a = 1;
    loop {
        let mut b = 1;
        loop {
            print!("{} × {} = {}\t", a, b, a * b);
            b += 1;
            if b > a { break };
        }
        println!();
        a += 1;
        if a > 9 { break };
    }
}