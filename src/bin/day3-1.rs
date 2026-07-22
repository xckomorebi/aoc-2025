use std::io;

fn get_joltage(s: String) -> i32 {
    let mut prev_max_digit: i32 = 0;
    let mut max_val: i32 = 0;

    for b in s.bytes() {
        let c = b as i32 - '0' as i32;
        if prev_max_digit == 0 {
            prev_max_digit = c;
            continue;
        }

        if prev_max_digit * 10 + c > max_val {
            max_val = prev_max_digit * 10 + c;
        }

        if c > prev_max_digit {
            prev_max_digit = c;
        }
    }

    max_val
}


fn main() {
    let mut result = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        result += get_joltage(line);
    }

    println!("{result}");
}
