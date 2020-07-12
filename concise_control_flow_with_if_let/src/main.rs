fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // this is equivalent to the above
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut coin = 0;
    match coin {
        Coin::Quarter(state) => println!("Quarter from state {}", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("Quarter from state {}", state);
    } else {
        count += 1;
    }
}
