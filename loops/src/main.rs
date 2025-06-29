use std::io;

fn main() {
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println! ("The result is {result}");

//     let mut counter = 0;
//     loop {
//         println!("Count now: {counter}");
//         if counter == 15 {
//             println!("Do you want to continue (y/n)");
//             let mut yes = String::new();
//             io::stdin()
//                 .read_line(&mut yes)
//                 .expect("Failed to read input");

//             if yes.trim() == "y"{
//             }
//             else {
//                 println! ("Counter end at {counter}");
//                 break;
//             }
//         }
//         if counter == 30 {
//             println! ("Counter end at {counter}");
//             break;
//         }
//         counter += 1;
//     }

let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
