fn main() {
    let _x = 2.0; // f64
    let _y: f32 = 2.0; // f32
    let _z = 1_000_000; // you can use _ as a visual separator

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;  // destructuring
    println!("x is {}", _x);
    let first_element = tup.0;
    let second_element = tup.1;
    println!("first_element is {}", first_element);
    println!("second_element is {}", second_element);

    // arrays
    // unlike tuples, all elements must have the same type
    // arrays have a fixed length
    let a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
    let index = 3;
    let value = a[index];
    println!("value is {}", value);

    let a = [3; 5]; // same as let a = [1, 2, 3, 4, 5];
    println!("a[2] is {}", a[2]);
}
