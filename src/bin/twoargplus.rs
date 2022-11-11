use std::env;

fn main() {
    if let [a, b] = &env::args().collect::<Vec<String>>()[1..].iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()[..] {
        println!("{}", a + b);
    }
}