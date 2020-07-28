// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "3"; // don't change this line
    let mut number = number.parse::<u32>().unwrap();
    println!("Number {}", number);
    number = 3;
    println!("Number {}", number);
}
