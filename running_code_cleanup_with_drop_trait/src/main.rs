struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`` !", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("My Stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("My other things"),
    };

    let e = CustomSmartPointer {
        data: String::from("Dropping early"),
    };

    println!("Smart pointers created");

    // we can't call e.drop() directly
    // we need to use std::mem::drop
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");

    // Rust calls drop() automatically when the instances go out of scope
    // variables are dropped in reverse order: d is dropped before c
}
