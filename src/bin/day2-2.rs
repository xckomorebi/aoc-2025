use std::io;

fn get_num_digit(id: i64) -> i8 {
    let mut cur = id;
    let mut len = 0;
    while cur != 0 {
        len += 1;
        cur /= 10;
    }

    len
}

fn is_invalid_id(id: i64) -> bool {
    let len = get_num_digit(id);

    for cand_len in 1..len {
        if len % cand_len == 0 && check_candidate_len(id, cand_len) {
            return true;
        }
    }

    false
}

fn check_candidate_len(id: i64, cand_len: i8) -> bool {
    let mut div = 1;

    for _ in 0..cand_len {
        div *= 10;
    }

    let check_num = id % div;
    let mut cur = id;

    while cur > 0 {
        if cur % div != check_num {
            return false;
        }
        cur /= div;
    }

    true
}

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let mut result = 0;

    for pair in buf.split(',') {
        let pair: Vec<&str> = pair.split('-').collect();

        let start_id: i64 = pair[0].trim().parse().unwrap();
        let end_id: i64 = pair[1].trim().parse().unwrap();

        for id in start_id..(end_id + 1) {
            if is_invalid_id(id) {
                result += id;
            }
        }
    }

    println!("{result}")
}
