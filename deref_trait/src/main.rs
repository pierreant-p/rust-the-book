use std::ops::Deref;

// Defining our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implement the deref trait for our smart pointer
impl<T> Deref for MyBox<T> {
    type Target = T;

    // deref must return a reference to the inner data
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // defining our own smart pointer
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // deref coercions
    let m = MyBox::new(String::from("Rust"));
    // Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>
    // without it we would have to write
    // hello(&(*m)[..]);
    hello(&m);
}
