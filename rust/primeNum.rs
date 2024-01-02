fn main(){
    let max = 100000;
    let mut v = Vec::new();
    v.push(2);

    'outer: for i in 3..max {

        for j in &v{
            if i % j == 0 { continue 'outer};
        }
        v.push(i)
    }

    for i in &v {
        println!("{i}");
    }
}