//if expressions
// fn main() {
//     let number = 243;
//     if number % 6 == 0 {
//         println!("number is divisible by 6.");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3.");
//     } else if number % 5 == 0 {
//         println!("number is divisible by 5.");
//     } else {
//         println!("number is not divisible by 6, 3, or 5.");
//     }
// }

//if expression in let statement
// fn main() {
//     let condition = true;
//     let number = if condition {"five"} else {"six"}; // both numbers has to by of same data type either
//     println!("The value of number is {number}")
// }

//if, loop, break 
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
     };
    println!("Result is {result}");
}
