fn main() {
    println!("Hello, world!");
    let x =2;
    if x<10{
        println!("x is less than 10");
} else if x<50 {
    println!("x is lower than 50");
} else {
    println!("x is greater or equal 50");
}

let mut n =0;
while n<10{ println!("{}",n);
    n += 1;
}
let mut n =0;
loop{
println!("{}", n);
n +=1;
 if n >10{
 break;
 }
}
} // closes main fn
