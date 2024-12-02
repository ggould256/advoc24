use std::env;

pub mod day1;

fn main() {
    if env::args().len() == 1 {
        println!("{}", day1::day1b());
    } else {
        for arg in env::args().skip(1) {
            println!(
                "{}",
                match arg.as_str() {
                    "day1" => day1::day1(),
                    "day1b" => day1::day1b(),
                    _ => panic!("Skipping unknown test {}", arg),
                }
            )
        }
    }
}
