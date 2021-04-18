mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn make(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::make("Rye");
    // Change our mind about what bread we'd like
    println!("{:?}", meal);
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);


    let mut meal2 = back_of_house::Breakfast::make("Brown");
    println!("{:?}", meal2);
    meal2.toast = String::from("White");
    println!("{:?}", meal2);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
