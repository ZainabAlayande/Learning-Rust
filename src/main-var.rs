fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

// use std::io;

// fn main() {

//     let a = [1,2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a [index];

//     println!("The value of the element at index {index} is: {element}");
// }