fn main() {
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("{}", first);

    // This creates references to portion of the string
    let _hello = &s[0..5]; // [start_index..end_index]. end_index is excluded
    let _world = &s[6..11];

    let _slice = &s[0..3];
    let slice = &s[..3]; // these 2 are equivalent

    let len = slice.len();
    let _slice = &s[2..len];
    let _slice = &s[2..]; // these 2 are equivalent

    let _slice = &s[0..len];
    let slice = &s[..]; // these 2 are equivalent
    println!("{}", slice);

    // Works with arrays too !
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
