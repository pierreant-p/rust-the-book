// in rust everything is private by default
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // use super to go to the parent module
        super::serve_order();
    }

    fn cook_order() {}

    // struct fields need to be made public on a case-by-case basis
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // this function is needed because season_fruit is private
        // if it didn't exist, we wouldn't be able to create an instance of Breakfast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peacher"),
            }
        }
    }

    // if an enum is public, all of it's variants are public
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("toast: {}", meal.toast);

    // this is forbidden because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("Apple");

    let order = back_of_house::Appetizer::Soup;
}
