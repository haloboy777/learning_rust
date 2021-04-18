fn main() {
    let string1 = String::from("abc");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

// use std::fmt::Display;

fn longest_with_an_announcement<'a>(x: &'a str, y: &'a str, ann: &str) -> &'a str
// where
//     T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
