use std::io;

fn count_num(val: i32, delta: i32) -> i32 {
    let mut result = 0;
    let mut delta = delta;

    if delta >= 100 || delta <= -100 {
        result += (delta / 100).abs();
        delta %= 100;
        if delta == 0 {
            return result;
        }
    }

    let cur = val + delta;

    if cur >= 100 {
        return result + 1;
    }

    if cur < 0 && val != 0 {
        return result + 1;
    }

    if cur == 0 && val != 0 {
        return result + 1;
    }

    result
}

fn main() {
    let mut zero_count = 0;
    let mut current_value = 50;

    for line in io::stdin().lines() {
        let s = line.expect("readline error");

        let delta: i32;
        if let Some(number_str) = s.strip_prefix("L") {
            delta = -number_str.parse::<i32>().expect("parse num error");
        } else if let Some(number_str) = s.strip_prefix("R") {
            delta = number_str.parse::<i32>().expect("parse num error");
        } else {
            continue;
        }

        zero_count += count_num(current_value, delta);

        current_value += delta;
        current_value %= 100;
        if current_value < 0 {
            current_value += 100;
        }
    }

    println!("{zero_count}")
}
