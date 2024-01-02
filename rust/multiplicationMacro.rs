macro_rules! run {
    () => {
        for i in 1..10 {
            for j in 1..(i + 1) {
                print!("{} × {} = {}\t", i, j, i * j);
            }
            println!()
        }
    }
}

fn main() {
   run!()
}
