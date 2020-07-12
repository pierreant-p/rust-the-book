fn main() {
    // creating a string
    let mut s = String::new();
    let data = "initial comments";
    let s = data.to_string();
    let s = "initial data".to_string();
    let s = String::from("initial string");

    // yay utf-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("नमस्ते");

    // updating a string
    let mut s1 = String::from("hello");
    s1.push_str(", world!");

    let mut s2 = String::from("hello");
    s2.push('f');

    // concatenation
    let s1 = String::from("hello");
    let s2 = String::from(", world");
    let s3 = s1 + &s2; // s1 is moved and can no longer be used
    let s4 = "!";
    let s5 = "Awesome";
    let hello = s3 + &s4 + &s5;
    let hello2 = format!("{}-{}", hello, s4);

    // indexing into strings
    let s1 = String::from("hello");
    // let h = s[0];  // does not work because it's ambiguous (would return bytes, not charcters)
    let h = &s1[0..2]; // works but use with caution. Can make the program panic.

    // iterating over strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}
