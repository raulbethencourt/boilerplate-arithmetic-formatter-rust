mod arithmetic_formater;
use arithmetic_formater::arithmetic_formater;

fn main() {
    let problems: Vec<&str> = ["32 + 698", "3801 - 2", "45 + 43", "123 + 49"].to_vec();
    
    println!("{:?}", arithmetic_formater(problems));
}
