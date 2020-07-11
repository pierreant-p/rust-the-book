fn main() {
    {
        let s = String::from("hello"); // s is valid from here

        // do stuff with s
        println!("{}", s);
    }
    // s goes out of scope and is no longer valid here

    {
        let x = 5;
        let y = x; // x value of x is copied and bound to y

        // when data is all on the stack, copying/cloning is fast (Copy trait)
        println!("x: {}, y: {}", x, y);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1; // s1 is moved to s2
                     // part allocated on the stack is copied and points to the same ptr on the heap, and then s1 in invalidated
                     // this prevents a double-freen when s1 and s2 go out of scope because s1 is already invalid

        // println!("s1: {}", s1);  // won't work because s1 is used after move
        println!("s2: {}", s2);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // heap data gets copied too
                             // clone might be expensive at runtime, hence it's made explicit
                             // any automatic copying can be assumed to be fast
        println!("s1: {}, s2: {}", s1, s2);
    }

    {
        let s = String::from("hello");
        takes_ownership(s); // s is moved into the function and not valid after

        let x = 5;
        makes_copy(x); // x is Copy so it's ok to use it after
    }

    {
        let s1 = gives_ownership(); // return value is moved to s1
        let s2 = String::from("hello");
        let s3 = takes_and_give_back(s2);
        println!("s1: {}, s3: {}", s1, s3);
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("length of '{}': {}", s2, len);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string is dropped here and memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope but nothing happens

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string // some_string is returned and moved out to the calling function
}

fn takes_and_give_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
