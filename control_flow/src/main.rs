fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("The number was not 0");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition { 3 } else { 5 };
    println!("number is {}", number);

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };
    println!("Result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("number is {}", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("value is {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("element is {}", element);
    }

    for x in (1..4).rev() {
        println!("x is {}", x);
    }

    println!("fibonacci is {}", fibonacci(20));

}

fn fibonacci(n: i32) -> i32 {
    let mut fminus1 = 0;
    let mut fminus2 = 1;
    let mut fnext = 0;

    for index in 1..n {
        println!("{}", index);
        fnext = fminus1 + fminus2;
        fminus1 = fminus2;
        fminus2 = fnext;
    }
    fnext
}
