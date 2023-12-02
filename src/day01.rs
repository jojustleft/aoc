use std::fs;

pub fn trebuchet(filepath: &str, debug: bool) -> i32 {
    let file_con = fs::read_to_string(filepath)
        .expect("Error reading file");

    let lines = file_con.split("\n");

    let mut final_sum = 0;

    for part in lines {
        let mut first_digit = ' ';
        for i in part.chars() {
            if i.is_digit(10) {
                first_digit = i;
                break;
            }
        }
        let mut second_digit = ' ';
        for i in part.chars().rev() {
            if i.is_digit(10) {
                second_digit = i;
                break;
            }
        }
        let mut curr_num = String::new();
        if first_digit.is_whitespace() || second_digit.is_whitespace() { continue; }
        curr_num.push(first_digit);
        curr_num.push(second_digit);

        if debug {println!("Checking line {}, found {}", part, curr_num);}

        final_sum += curr_num.parse::<i32>().unwrap();
    }

    if debug { println!("Final result {final_sum}"); }

    final_sum
}