use std::fs;
use std::collections::HashMap;

fn read_arrays(filepath: &str) -> (Vec<i32>, Vec<i32>) {
    let file_con = fs::read_to_string(filepath)
        .expect("Error reading file");

    let lines = file_con.split("\n");

    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();

    for l in lines {
        let curr = l.trim().split("   ").collect::<Vec<&str>>();
        arr1.push(curr[0].parse::<i32>().unwrap());
        arr2.push(curr[1].parse::<i32>().unwrap());
    }
    arr1.sort();
    arr2.sort();

    (arr1, arr2)
}

pub fn historian(filepath: &str, debug: bool) -> i32 {
    let (arr1, arr2) = read_arrays(filepath);
    let mut dist = 0;

    if arr1.len() != arr2.len(){
        panic!("Array sizes differ");
    }

    for i in 0..arr1.len() {
        dist += (arr1[i] - arr2[i]).abs();
    }
    
    if debug { println!("Day 1: Final result {dist}"); }
    dist
}

pub fn historian_p2(filepath: &str, debug: bool) -> i32 {
    let (arr1, arr2) = read_arrays(filepath);
    let mut similarity = 0;

    let mut arr2_counter = HashMap::new();
    for el in arr2 {
        *arr2_counter.entry(el).or_insert(0) += 1;
    }

    for el in arr1{
        let curr_count = arr2_counter.get(&el).unwrap_or(&0);
        similarity += el * curr_count;
    }

    if debug { println!("Day 1 part 2: Final result {similarity}"); }
    similarity
}