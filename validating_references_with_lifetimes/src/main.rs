use std::fmt::Display;

fn main() {
    // Code does not work because because x does not live long enough
    //
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }

    // Lifetime annotation in struct definitions
    let novel = String::from("A long time ago. In a far away galaxy ...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime
    // s can live for the duration of the progam
    // all string literals have a static lifetime
    let s: &'static str = "I have a static lifetime";
}

// We're telling  Rust that the lifetime of the reference returned
// by the longest function is the same as the smaller of the lifetimes
// of the references passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// y doesn't need to be annotated because it doesn't have
// any relationship to the lifetime of the returned value
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// An instance of ImportantExcerpt cannont outlive the reference it holds
// in it's part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Everything together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
