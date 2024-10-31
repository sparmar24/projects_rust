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
fn main() {
    let rect = (30, 50);
    println!("area of rectangle is {} squrare pixels",
    area(rect)
);

fn area(dimesions: (u32, u32)) -> u32 {
        dimesions.0 * dimesions.1
    }
}
