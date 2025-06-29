use std::io;
fn main() {
    //addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    //multiplicaiton
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2; //f64 - rust will automatically assigned the data types

    let quotient_2: f32 = 56.7 / 32.2; //f32

    // remainder 
    let remainder = 43 % 5;

    println! ("Sum: {sum}");
    println! ("Diff: {difference}");
    println! ("Product: {product}");
    println! ("Quotient: {quotient}");
    println! ("Quotient_2: {quotient_2}");
    println! ("Remainder: {remainder}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println! ("Char: {c} {z} {heart_eyed_cat}");

    // tuple
    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    // println! ("The tuple value is {_x}, {_y}, {_z} ");
    println! ("The tuple value is {}, {}, {} ", tup.0, tup.1, tup.2);
    println! ("The tuple value is {:?}", tup);
    let _y = tup.1; 
    println! ("The value of y is {_y}");
    let first_tup = tup.0;
    println! ("The value of first tuple is {first_tup}");

    // array
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let first_month = months[0];
    // println! ("The first month is {months[0]}"); //you can't directly call array on println
    println! ("The first month is {first_month}");
    println!("All of the months in a year: {:?}", months);

    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index from 1 to 5.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
