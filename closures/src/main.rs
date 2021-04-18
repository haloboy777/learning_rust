fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    println!("{}", s);
}
