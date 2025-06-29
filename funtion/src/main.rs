use std::io;

fn main() {
    println!("function");
    println!("Plesae enter a number: ");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to enter number");

    // Trim the newline and parse the string to a number (e.g., i32)
    let number: i32 = number.trim().parse().expect("Please enter a valid number");
    print_number(number);
}

fn print_number(x: i32) {
    println! ("Number entered is {:?}", x);
}