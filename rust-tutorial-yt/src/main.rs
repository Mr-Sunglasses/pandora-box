// use std::{cmp::Ordering, io}; used for gussing_game

// use rand::Rng; used for gussing_game


// This file contains the examples of rustbook.
fn main() {
    println!("Hello, world!"); // Hello, world!



    // data_types();

    // variables_and_mutability();

    // guessing_game();
}


// #[allow(unused)]
// fn data_types() {
//     /*
//         Scalar Types - represents only a single value
//         - int
//         - float
//         - bool
//         - char
//     */

//     // integer
//     let my_number = 100_000; // default i32
    
//     let this_size_by_system: usize = 120_000;


//     // integer overflow
//     let a_u8_int: u8 = 255u8;
//     // wrapping_* Methods
//     let y = a_u8_int.wrapping_add(1); // y = 0 ie. 266 = 0 267 = 1.... 

//     // checked_* Methods

//     // floating point
//     let f = 3.15; // default f64

//     // boolean
//     let t = true;
//     let f: bool = false;

//     let character: char = 'a';

//     // tuple
//     /*
//         - have fixed length ( can't grow or shrink )
//         - have predefined types and can be a consit of any type
//     */ 

//     let tup = (1, "Hello", 3.14, true);

//     let tupa: (i32, u8, bool, f64, bool, &str) = (123, 255, true, 3.145, false, "World");

//     let (x, y, z, a) = tup;

//     let pi = tup.3;

//     println!("{}", tup.0); // prints what at tup[0] index

//     // array type
//     /*
//         - fixed length ( can't grow or shrink )
//         - contains elements of same type
//         - store data in the stack
//     */

//     let arr = [1,2,3,4,5];
//     let my_array: [i32; 5] = [2, 4, 6, 8, 10];

//     let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];

//     let copy = ['a'; 5]; // result ['a', 'a', 'a', 'a', 'a']

//     // accessing he elment of an arrya
//     let first_month = months[0];

//     let last_moth = months[months.len() - 1]; // 12 - 1
    
//     // let out_of_bound_length = months[13];




// }

// fn variables_and_mutability() {
//     const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

//     println!("{}", THREE_HOURS_IN_SECONDS);

//     let mut x = 5; // using mut to make it mutable
//     println!("The value of x is: {}", x);
//     x = 10;
//     println!("The value of x is: {}", x);

//     // shadowing

//     let x = 5; // Reference 1

//     println!("Value of x: {}", x);

//     let x = x + 1; // Reference 2, overshadowing reference 1

//     println!("Value of x: {}", x);

//     {
//         let x = x * 2;

//         println!("Value of x: {}", x); // Reference 3, but in it's own scope {}

//         // note: the above not overshadows any reference but take the value of other reference and only works witin it's scope.
//         // it ends when the scope is over
//     }
//     println!("Value of x: {}", x);
// }

// Programming a gussing game
// fn guessing_game() {
//     println!("Guess the Number!");

//     let secret_number = rand::rng().random_range(1..=100); // generate a random number b/w 1 to 100

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         /*
//             Used for I/O
//             - Returns, value or error
//             - expect (if there is error show this message)
//         */
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         /*
//             Parse the guess to a number of u32 (unsigned 32 bit)
//                 - if matches keep it
//                 - if not show error and continue
//         */
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("You guess must be a number of integer nothing else");
//                 continue;
//             },
//         };

//         println!("You guesses {}", guess);

//         /*
//             - Evaluating the guess and secret_number
//             - alternatively can also be done by if and else.
//         */
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small"),
//             Ordering::Greater => println!("Too big"),
//             Ordering::Equal => {
//                 println!("You win!.");
//                 break;
//             }
//         }

//         // if else approach
//         // if guess < secret_number {
//         //     println!("To small");
//         // } else if guess > secret_number {
//         //     println!("Too big")
//         // } else if guess == secret_number {
//         //     println!("You win!.");
//         //     break;
//         // } else {
//         //     // gracefully handle error if any
//         //     eprintln!("Error happend!");
//         //     continue;
//         // }
//     }
// }

// let mut var = String::new();

//     io::stdin().read_line(&mut var).expect("err");

//     let x: bool = var.trim().parse().expect("Not a boolean");

//     println!("{}", x);
