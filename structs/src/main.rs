// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//calculate area using tuple
// fn main() {
//     let rect = (30, 50);
//     println!("area of rectangle is {} squrare pixels",
//     area(rect)
// );

// fn area(dimesions: (u32, u32)) -> u32 {
//         dimesions.0 * dimesions.1
//     }
// }

//refactoring with structs
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 60,
//     };
//     println!(
//         "The area of the rectanlge is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

//add debugger before struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {rect1:?}");
    area(&rect1);
    println!("the area is {} square pixels", area(&rect1));
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
