
fn main() {
    let args : Vec<String> = std::env::args().collect();

    let fibonacci_param = &args[1];
    let fibonacci_number = fibonacci_param.parse::<i32>().unwrap();

    println!("{}", compute_fibonacci(fibonacci_number));
}

fn compute_fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => compute_fibonacci(n-1) + compute_fibonacci(n-2)
    }
}
