fn main() {
    // Fib sequence in rust
    fn fib(a:i32) -> i32{
        if (a<=2){
            return a
        }
        else{
            return fib(a-1) + fib(a-2);
        }
    }
    println!("result {}",fib(5))
}

