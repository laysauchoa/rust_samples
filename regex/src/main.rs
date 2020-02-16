
use regex::Regex;
fn main() {
    //let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let re = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
    println!("{:?}",assert!(re.is_match("0e7da276c87fb5c58f43d7cfb5011996")));
}
