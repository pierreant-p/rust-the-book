fn main() {
    // vectors puts all values next to each other in memory
    let v1: Vec<i32> = Vec::new();

    // you can also use the macro
    // rust will infer the type from the values
    let v2 = vec![1, 2, 3];

    // updating a vector
    let mut v3: Vec<i32> = Vec::new();
    v3.push(5);
    v3.push(6);

    {
        let v4 = vec![1, 2, 3];
    } // v4 goes out of scope and is freed here
      // when the vector is dropped, all of its contents are dropped

    // reading from a vector
    let v5 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v5[2];

    // get() returns an Option<&T>
    let four = v5.get(4);

    // this panics
    // let does_not_exist = v5[100];

    // this doesn't panic (returns None)
    let does_not_exist = v5.get(100);

    match v5.get(2) {
        Some(two) => println!("{}", two),
        None => println!("No such element"),
    }

    //
    let mut v = vec![12, 1, 4, 19, 37];
    let first = &v[0];
    // this wouldn't work because you can't have a mutable and immutable reference in the same scope
    // v.push(6);
    println!("{}", first);

    // iterate over vector
    for i in &v {
        println!("{}", i);
    }

    // iterate and mutate
    for i in &mut v {
        // needs dereferencing to get the value
        *i += 50;
    }

    // use enum to have vector with different types
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Cell::Int(3),
        Cell::Float(1.0),
        Cell::Text(String::from("hello")),
    ];
}
