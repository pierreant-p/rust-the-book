mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


// bring path into scope
use crate::front_of_house::hosting;

// this would also work
// use self::front_of_house::hosting;

// this would also work but is not idiomatic
// because it makes it unclear that the function is not defined locally.
// use self::front_of_house::hosting::add_to_waitlist;

// however, for struct and enums, it's idiomatic to specify the full path
use std::collections::HashMap;


// unless there is a namespace collision
// in this case using the parent module allows to desambiguate
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }


// we can also rename
use std::io::Result as IOResult;

// re-exporting external code can now call the add_to_waitlist
// function using hosting::add_to_waitlist
// pub use crate::front_of_house::hosting;

// import external dependencies
use rand::Rng;

// use can be combined
use std::io;
use std::io::Write;

// is similar to
use std::{self, Write};

use std::cmp::Ordering;
use std::io;

// is similar to
use std::{cmp::Ordering, io};

// glob operator to import everything
// use with caution
use std::collections::*;

fn main() {
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret = rand::thread_rng().gen_range(1, 100);
    println!("{}", secret);
}
