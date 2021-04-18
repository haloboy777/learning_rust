use std::collections::HashMap;

fn main() {
    let s1 = String::from("hello world");
    let h = s1.get(0..3);
    let test = vec![1, 2, 3, 4, 5];
    match h {
        None => println!("There is no string"),
        Some(el) => println!("The string slice contains: {:?}", el),
    }
    println!("{:?}", test);
    let mut hash = HashMap::new();
    hash.insert(String::from("Blue"), 10);
    hash.insert(String::from("Yellow"), 50);
    println!("{:?}", hash);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);
}
