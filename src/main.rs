use std::env;

mod parsing;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    if env::args().len() == 1 {
        println!("{}", day1::day1b(None));
    } else {
        for arg in env::args().skip(1) {
            println!(
                "{}",
                match arg.as_str() {
                    "day1" => day1::day1(None),
                    "day1b" => day1::day1b(None),
                    "day2" => day2::day2(None),
                    "day2b" => day2::day2b(None),
                    "day3" => day3::day3(None),
                    "day3b" => day3::day3b(None),
                    "day4" => day4::day4(None),
                    "day4b" => day4::day4b(None),
                    "day5" => day5::day5(None),
                    "day5b" => day5::day5b(None),
                    "day6" => day6::day6(None),
                    "day6b" => day6::day6b(None),
                    
                    _ => panic!("Skipping unknown test {}", arg),
                }
            )
        }
    }
}
