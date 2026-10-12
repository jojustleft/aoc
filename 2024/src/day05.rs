use std::fs;
use std::collections::HashMap;

/*
Unfortunately, the rules are cyclic, meaning we cannot simply rely on topological sorting and bypass any
update-specific logic.
They are, however, cycle free within each update. So we can filter the rule set for only entries
relevant to a given update, create the expected list, and compare with the obtained one.
*/

fn get_expected_list(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> Vec<i32> {
    let mut update_order = HashMap::new();
    for rule in rules {
        let (a, b) = rule.clone();
        // Only rules that contain elements in the current update
        if !update.contains(&a) || !update.contains(&b) { continue; }
        
        if let Some(order) = update_order.get(&a) {
            update_order.insert(a, order - 1);
        }
        else { update_order.insert(a, -1); }
        
        if let Some(order) = update_order.get(&b) {
            update_order.insert(b, order + 1);
        }
        else { update_order.insert(b, 1); }
    }
    let mut update_template: Vec<(i32, i32)> = update_order.into_iter().collect();
    update_template.sort_by_key(|el| el.1);
    let update_template: Vec<i32> = update_template.into_iter()
        .map(|(s1, _)| s1)
        .collect();

    update_template.clone()
}

pub fn rule_check(filepath: &str, matches: bool, debug: bool) -> i32 {
    let file_con = fs::read_to_string(filepath)
        .expect("Error reading file");

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in file_con.lines() {
        if line.contains('|') {
            let mut parts = line.split('|')
                .map(|s| s.trim().parse::<i32>());
            if let (Some(Ok(a)), Some(Ok(b))) = (parts.next(), parts.next()) {
                rules.push((a, b));
            } else {
                panic!("Error parsing line: {}", line);
            }
        }
        else if line.contains(',') {
            let res: Result<Vec<i32>, _> = line.split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect();
            match res {
                Ok(update) => updates.push(update),
                Err(e) => panic!("Error parsing line: {line} {e}")
            };
        }
    }

    let mut update_sum = 0;
    for update in updates {
        let current_expected: Vec<i32> = get_expected_list(&rules, &update);
        if current_expected == update && matches {
            let mid = update.len().div_ceil(2) - 1;
            update_sum += update[mid];
        }
        // expected is already the sorted version of incorrectly-ordered updates
        else if current_expected != update && !matches {
            let mid = current_expected.len().div_ceil(2) - 1;
            update_sum += current_expected[mid];
        }
    }

    if debug {
        println!("Day 5: Final result {update_sum}")
    }

    update_sum
}
