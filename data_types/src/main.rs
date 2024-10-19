use std::io;

fn main() {
    
    // let t: (u32, u32, u32, u32, u32) = (11, 12, 13, 14, 15);
    // let t = (11, 12, 13, 14, 15);
    // println!("Please enter index for a tuple.");
    // let mut tindex = String::new();

    // io::stdin()
    //     .read_line(&mut tindex)
    //     .expect("Failed to read line");

    // let tindex: usize = tindex
    //     .trim()
    //     .parse()
    //     .expect("Entered index was not a number");

    // let telement: usize = t.0;

    // println!("The value of the element at index {tindex} is: {telement}");

    let a = [1, 2, 3, 4, 5];
    println!("Please enter index of an array");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: usize = a[index];

    println!("The value of the element at index {index} is: {element}");

    // //basic introduction of data types 
    // let _sum = 5 + 10;
    // let _difference = 95.5 - 4.3;
    // let _product = 4 * 30;
    // let _quotient = 56.7 / 32.2;
    // let _truncated = -5 / 3;
    // let _remainder = 43 % 5;
    // println!("remainder is {_remainder}");
    // 
    // let _t = true;
    // let _f: bool = false;

    // let tup: (i32, f64, u8) = (500,6.4,1); //tuple definition
    // let (x,y,z) = tup;
    // println!("the value of x is: {x}");    
    // println!("the value of y is: {y}");    
    // println!("the value of z is: {z}");    
    // let _five_hundred = tup.0; //get first element of tuple
    // println!("the value of x is: {x}");    
    // let _six_point_four = tup.1;pyqt
    // let _one = tup.2;


}

