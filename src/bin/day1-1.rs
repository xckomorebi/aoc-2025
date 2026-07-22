use std::io;

fn main() {
    let mut zero_count = 0;
    let mut current_value = 50;

    for line in io::stdin().lines() {
        let s = line.expect("readline error");

        if let Some(number_str) = s.strip_prefix("L") {
            current_value -= number_str.parse::<i32>().expect("parse num error");
        } else if let Some(number_str) = s.strip_prefix("R") {
            current_value += number_str.parse::<i32>().expect("parse num error");
        } else {
            continue;
        }

        if current_value % 100 == 0 {
            zero_count += 1;
        }
    }

    println!("{zero_count}")
}
