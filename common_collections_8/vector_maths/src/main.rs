use std::collections::HashMap;

fn main() {
    let mut cool_nums = vec![5, 2, 66, 12, 53, 234, 33, 11, 23, 15, 12, 643, 34];

    // Get mode
    let mut number_frequence = HashMap::new();
    for num in &cool_nums {
        number_frequence
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    if let Some(&number) = number_frequence
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
    {
        println!("Mode is {number}");
    }

    // Get Mean
    let _ = &cool_nums.sort(); // Sort by alphabetical order
    let mean_value = cool_nums[cool_nums.len() / 2];
    println!("Mean num is... {}", { mean_value });
}
