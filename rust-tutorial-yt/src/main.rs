// use std::{cmp::Ordering, io}; used for gussing_game

// use rand::Rng; used for gussing_game

// This file contains the examples of rustbook.
fn main() {
    println!("Hello, world!"); // Hello, world!

    data_types();

    // variables_and_mutability();

    // guessing_game();
}

fn data_types() {
    /*
        Scalar Types - represents only a single value
        - int
        - float
        - bool
        - char
    */

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

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
