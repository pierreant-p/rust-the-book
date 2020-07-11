fn main() {
    let s1 = String::from("hello");
    let r = &s1; // create a reference to s1
    // it refers to the value of s1 but does not own it
    // hence the value will not be dropped whrn the value goes out of scope
    let len = calculate_length(r);
    println!("len of '{}' is {}", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // This doesn't work because you can't have more than one mutable ref to a piece of data
    // let r1 = &mut s;
    // let r2 = &mut s;


    // This is NOT ok (only one mutable reference)
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{}, {}, {}", r1, r2, r3);

    // This is ok
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1 and r2 are only valid up to here because they are not used after

    let r3 = &mut s;
    println!("{}", r3);
}


// This implemenation uses a reference
// Having references as function parameters is called borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope. Because it does not have ownership of what it refers to, nothing happens


// Code below doesn't work because you can't modify something you borrow
// reference are immutable by default

// fn change(some_string: &String) {
//     some_string.push_str(", world!");
// }


fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// Code below is invalid because s goes out of scope, creating a pointer to a non-existing string

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope here. &s would be invalid
