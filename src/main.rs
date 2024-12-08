use std::env;

mod parsing;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    if env::args().len() == 1 {
        println!("{}", day1::day1b(None));
    } else {
        for arg in env::args().skip(1) {
            let result: i64 = match arg.as_str() {
                "day1" => day1::day1(None).into(),
                "day1b" => day1::day1b(None).into(),
                "day2" => day2::day2(None).into(),
                "day2b" => day2::day2b(None).into(),
                "day3" => day3::day3(None).into(),
                "day3b" => day3::day3b(None).into(),
                "day4" => day4::day4(None).into(),
                "day4b" => day4::day4b(None).into(),
                "day5" => day5::day5(None).into(),
                "day5b" => day5::day5b(None).into(),
                "day6" => day6::day6(None).into(),
                "day6b" => day6::day6b(None).into(),
                "day7" => day7::day7(None),
                "day7b" => day7::day7b(None),
                "day8" => day8::day8(None),
                "day8b" => day8::day8b(None),
                
                _ => panic!("Skipping unknown test {}", arg),
            };
            println!("{} output: {}", arg, result)
        }
    }
}
