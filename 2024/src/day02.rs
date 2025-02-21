use std::fs;

fn check_report(arr: Vec<i32>, dampener: bool) -> bool {
    let first_diff: Vec<i32> = arr
        .windows(2)
        .map(|x| x[1] - x[0]).collect();

    let decreasing = first_diff[0] < 0;
    for (i, el) in first_diff.into_iter().enumerate() {
        if (el == 0) | ((el < 0) != decreasing) | (el.abs() > 3) {
            match dampener {
                true => {
                    let splice_first = match (el < 0) != decreasing {
                        false => false,
                        true => {
                            let mut ar0 = arr.clone();
                            ar0.remove(0);
                            check_report(ar0, false)
                        }
                    };
                    let mut ar1 = arr.clone();
                    ar1.remove(i);
                    let mut ar2 = arr.clone();
                    ar2.remove(i + 1);
                    return splice_first |
                           check_report(ar1, false) |
                           check_report(ar2, false)
                },
                false => return false
            };
        }
    }
    return true;
}

pub fn reports(filepath: &str, dampener: bool, debug: bool) -> i32 {
    let file_con = fs::read_to_string(filepath)
        .expect("Error reading file");

    let lines = file_con.split("\n");
    let mut num_safe = 0;
    for l in lines {
        let curr = l.trim().split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        
        num_safe += check_report(curr, dampener) as i32;
    }
    
    if debug { 
        match dampener {
            false => println!("Day 2: Final result {num_safe}"),
            true => println!("Day 2 part 2: Final result  {num_safe}")
        };
    }
    num_safe
}