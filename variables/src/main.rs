fn main() {
    let x = 5; //assign value 5 to x
    println!("The value of x is: {x}");

    const NUM: u32 = 60; //declare constant number

    let x = x+1+NUM; //assign x again and add 1 and constant to x value
    println!("The value of x is: {x}");
    
    {
        let x = x*4; // assign x again and multiply by 4
        println!("The value of x in inner loop is: {x}");
    }
    
    println!("The value of x is: {x}"); //will give the same value as was outside the loop

}
