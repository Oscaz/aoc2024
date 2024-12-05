use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Rule {
    left: u16,
    right: u16
}

fn main() {
    let input = std::fs::read_to_string("day05.txt").unwrap();
    let split: Vec<&str> = input.split("\r\n\r\n").collect();
    let rules: Vec<String> = split.get(0).unwrap().split("\r\n").map(|s| s.to_owned()).collect();
    let updates: Vec<String> = split.get(1).unwrap().split("\r\n").map(|s| s.to_owned()).collect();
    
    let rules: Vec<Rule> = rules.iter().map(|s| {
        let split: Vec<&str> = s.split("|").collect();
        let left = split.get(0).unwrap().parse::<u16>().unwrap();
        let right = split.get(1).unwrap().parse::<u16>().unwrap();
        Rule {
            left,
            right
        }
    }).collect();

    let updates: Vec<Vec<u16>> = updates.iter().map(|s| {
        s.split(",").map(|s| s.parse::<u16>().unwrap()).collect()
    }).collect();

    // maps some number A such that if it must be before some other number, its deep map boolean will be true
    let mut rule_map: HashMap<u16, HashMap<u16, bool>> = HashMap::new();
    rules.iter().for_each(|rule| {
        let mut temp_map = HashMap::new();
        let map = rule_map.get_mut(&rule.left).unwrap_or(&mut temp_map);
        let mut insert_map = map.clone();
        insert_map.insert(rule.right, true);
        rule_map.insert(rule.left, insert_map);
    });

    let mut safe_count: i32 = 0;
    let mut middle_count = 0;
    let mut incorrect_updates: Vec<Vec<u16>> = vec![];
    updates.iter().for_each(|update| {
        let mut safe = true;
        let mut ongoing_map: HashMap<u16, bool> = HashMap::new();
        update.iter().for_each(|num| {
            let safe_check = rule_map.get(num).map(|map| {
                !ongoing_map.keys().into_iter().find(|ongoing| map.contains_key(*ongoing)).is_some()
            }).unwrap_or(true);
            ongoing_map.insert(*num, true);
            if !safe_check {
                safe = false;
            }
        });
        if safe {
            safe_count += 1;
            middle_count += update.get(update.len() / 2).unwrap();
        } else {
            incorrect_updates.push(update.clone());
        }
    });
    dbg!(safe_count);
    dbg!(incorrect_updates.len());
    dbg!(middle_count);

    incorrect_updates.iter_mut().for_each(|update| {
        let mut i = 0;
        loop {
            
            if i == update.len() {
                break;
            }

            let num = *update.get(i).unwrap();
            if i == 0 {
                i += 1;
                continue;
            }

            rule_map.get(&num).map(|map| {
                let to_update = update.windows(i).next().unwrap().iter().find(|&up_num| {
                    map.contains_key(up_num)
                }).map(|up_num| update.iter().position(|n| n == up_num).unwrap());
                if let Some(to_update) = to_update {
                    let up_num = update.get(to_update).unwrap().clone();
                    update.swap(to_update, i);
                    i = 0;
                    println!("Swapping {} with {} due to rule {}|{}: {:?}", num, up_num, num, up_num, update);
                }
            });

            // if i == update.len() - 1 {
            //     let mut safe = true;
            //     let mut ongoing_map: HashMap<u16, bool> = HashMap::new();
            //     update.iter().for_each(|num| {
            //         let safe_check = rule_map.get(num).map(|map| {
            //             !ongoing_map.keys().into_iter().find(|ongoing| map.contains_key(*ongoing)).is_some()
            //         }).unwrap_or(true);
            //         ongoing_map.insert(*num, true);
            //         if !safe_check {
            //             safe = false;
            //         }
            //     });
            //     if !safe {
            //         i = 0;
            //     }
            // }
            i += 1;
        }
        
    });

    let mut safe_count: i32 = 0;
    let mut middle_count = 0;
    let mut bad_count = 0;
    incorrect_updates.iter().for_each(|update| {
        let mut safe = true;
        let mut ongoing_map: HashMap<u16, bool> = HashMap::new();
        update.iter().for_each(|num| {
            let safe_check = rule_map.get(num).map(|map| {
                !ongoing_map.keys().into_iter().find(|ongoing| map.contains_key(*ongoing)).is_some()
            }).unwrap_or(true);
            ongoing_map.insert(*num, true);
            if !safe_check {
                safe = false;
            }
        });
        if safe {
            safe_count += 1;
            middle_count += update.get(update.len() / 2).unwrap();
        } else {
            // incorrect_updates.push(update.clone());
            bad_count += 1;
        }
    });

    // dbg!(incorrect_updates);
    let mut middle_count = 0;
    incorrect_updates.iter().for_each(|update| {
        middle_count += update.get(update.len() / 2).unwrap();
    });
    dbg!(middle_count);
    dbg!(bad_count);
}