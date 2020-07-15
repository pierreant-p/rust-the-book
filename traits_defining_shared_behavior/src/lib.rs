pub trait Summary {
    // implementation is required
    fn summarize_author(&self) -> String;

    // default implemenation
    fn summarize(&self) -> String {
        format!("Read more from {} ...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// this function accepts any type that implements Summary as argument
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// same as above, but item1 and item2 must have the same type
pub fn notify2<T: Summary>(item1: T, items2: T) {
    // something
}

// more than one trait
pub fn notify2(item: impl Summary + Display) {
    // something
}

// more complex cases can be specified with a where clause
fn some_function<T, U>(t: t, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // something
}

// a function that return a value of some type that implements a trait
fn returns_summarizable() -> impl Summary {
    // something
}
// However, the function can only return a single type.
// This doesn't work
fn returns_something() -> impl Summary {
    if (/*something*/) {
        Tweet {
            // whatever
        }
    } else {
        NewsArticle {
            // whatever
        }
    }
}
