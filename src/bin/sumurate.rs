use std::env;

fn main() {
    let arg_nums = &env::args().collect::<Vec<String>>()[1..].iter().map(|s| s.parse::<i16>().unwrap()).collect::<Vec<i16>>();
    let sum = arg_nums.iter().fold(0, |sub, item| sub + item);
    println!("{sum}");
}