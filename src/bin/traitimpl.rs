trait Callable {
    type CallReturn: Sized;

    fn call() -> Self::CallReturn;
}

struct Monster ();

impl Callable for Monster {
    type CallReturn = String;

    fn call() -> String {
        "aaaaaaaaaaaaaa".to_string()
    }
}


fn main() {
    println!("{}", Monster::call())
}