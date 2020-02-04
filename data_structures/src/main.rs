fn main() {
    println!("This is a list in rust");
    let animals = ["rabbit", "dog", "cat"];
    //println!("{:?}", animals);
    for a in animals.iter(){
        println!("animal {}", a);

    }
println!("len of the list {}",animals.len());
struct Dog { //this is how a class should look in rust
        eyes: u32,
        legs: u32
}
impl Dog {
    fn print_me(&self){ //implementing a method
        println!("eyes {},  legs {}", self.eyes, self.legs);
    }
}
let dog1 = Dog{eyes:2, legs:4}; 
dog1.print_me();
}
