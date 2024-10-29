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
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter;
//         }
//      };
//     println!("Result is {result}");
// }

// multiple loops and break statement
// fn main() {
//     let mut count = 0; //initialize the count variable
//     'counting_up: loop { //labelling the loop
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 5 { //loop will run from 10 to 5 in decreasing order
//                 break; //exits inner loop
//             }
//             if count == 6 { //and also counts upto 6
//                 break 'counting_up; //exits the outer loop
//             }
//             remaining -=1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }
//
// //while loop
// fn main() {
//     let mut count = 50; //set the count value to fix number
//     while count != 10 { //start while loop
//         println!("count is {count}");
//         count -= 10; //subtract 10 every time
//     }
//     println!("while loop ends !!!")
// }

//while loop on elements of an array
// fn main() {
//     for_loop();

//     let a = [10, 20, 30, 40, 50, 60, 70, 80, 90];
//     let mut index = 0;
//     while index < 9 {
//         println!("the value: {}", a[index]);
//         index += 1;
//     }
// }

// fn for_loop() {

//     let b = [1,2,3,4,5];
//     for i in b {
//         println!("the value is: {i}");
//     }
// }

//countdown using for loop and rev function
fn main() {
    for number in (1..9).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!")
}
