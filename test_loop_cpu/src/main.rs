use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut num: i64;
    loop {
        num = rand::thread_rng().gen_range(1..11);
        num = num * 3 - 22 / 3;
        println!("{}", num);
        if num == 8 {
            break;
        }
        // thread::sleep(num);
    }
}
