fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_param(5, 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    let x = five();
    println!("The value of x is {}", x);

    let x = plus_one(7);
    println!("The value of x is {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
