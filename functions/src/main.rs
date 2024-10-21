fn main() {
    add_numbers();
    sum_function(6);
}

fn add_numbers(){
    let x = 4;
    let y = 5;
    let sum = x+y;
    println!("Sum of x and y is: {sum}");
}
fn sum_function(s: i32) -> i32{
    let s = s + 1;
    println!("The increment of a number is: {s}");
    return s;
}
