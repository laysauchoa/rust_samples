mod fibonnacci;
use std::io;
fn main() {
    loop {
        println!("Please enter the desired position for fib sequence");
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    let fib_res = fibonnacci::fib(n);
    println!("For position n: {}", fib_res);
}
}

