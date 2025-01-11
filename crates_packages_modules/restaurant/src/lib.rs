mod front_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appertiser {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    /*
        path crate::front_of_house::hosting::add_to_waitlist(); // abs path
        front_of_house::hosting::add_to_waitlist(); // rel path
    */

    // Order a breakfast with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about the bread we want
    meal.toast = String::from("Brown");

    println!("I'd like {} toast please.", meal.toast);

    let order1 = back_of_house::Appertiser::Soup;
    let order2 = back_of_house::Appertiser::Salad;
}
