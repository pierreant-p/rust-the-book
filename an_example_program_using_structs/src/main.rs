// We are writing a program the computes area
#[derive(Debug)]
struct Rect {
    height: u32,
    width: u32,
}

fn main() {
    // // first method
    // let width1 = 50;
    // let heigth1 = 30;

    // println!("The area is {}", area(width1, heigth1));

    // using tuples
    // let rect1 = (30, 50);
    // println!("The area is {}", area(rect1));

    // using structs
    let rect1 = Rect {
        height: 30,
        width: 50,
    };
    println!("The area is {}", area(&rect1));
    println!("The rect is {:?}", rect1); // use Debug formatter
    println!("The rect is {:#?}", rect1); // use Debug formatter with pretty-print
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rect: &Rect) -> u32 {
    rect.height * rect.width
}
