use std::fs;
use regex::Regex;

pub fn salvage(filepath: &str, trimmer: bool, debug: bool) -> i32 {
    let line = fs::read_to_string(filepath)
        .expect("Error reading file")
        .replace("\n", "");
    let re = Regex::new("mul\\(([0-9]{1, 3},[0-9]{1, 3}\\))").unwrap();

    let final_line = match trimmer {
        true => {
            line.split("do()")
                .map(|x| {
                     x.split("don't()")
                      .collect::<Vec<&str>>()[0]
                })
                .collect::<Vec<&str>>()
                .join("")
        },
        false => line
    };

    let mut total = 0;
    let results: Vec<_> = re.find_iter(final_line.as_str())
        .map(|entry| entry.as_str())
        .collect();

    for el in &results {
        let temp = &el[4..el.len()-1].split(',').collect::<Vec<&str>>();
        total += temp[0].parse::<i32>().unwrap() * (temp[1].parse::<i32>().unwrap());
    }
    if debug { 
        match trimmer {
            false => println!("Day 3: Final result {total}"),
            true => println!("Day 3 (part 2): Final result {total}")
        }
    }
    total
}
