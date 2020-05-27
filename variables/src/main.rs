fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // shadowing
    let y = 1;
    let y = y + 3;
    println!("The value of y is {}", y);

    let spaces = "      ";
    // spaces = spaces.len();  <-- compile error
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);
}
