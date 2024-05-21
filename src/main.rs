use std::collections::HashMap;

fn median(numbers: &mut Vec<i32>) -> f64 {
    // Sort the numbers
    numbers.sort();
    let len = numbers.len();
    if len % 2 == 0 {
        // If even, the median is the average of the two middle numbers
        (numbers[len / 2 - 1] + numbers[len / 2]) as f64 / 2.0
    } else {
        // If odd, the median is the middle number
        numbers[len / 2] as f64
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    // Count occurrences of each number
    for &num in numbers.iter() {
        let count = occurrences.entry(num).or_insert(0);
        *count += 1;
    }

    // Find the number with the maximum occurrences
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .expect("Cannot compute mode of an empty list")
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4];

    let med = median(&mut numbers);
    let mod_val = mode(&numbers);

    println!("Median: {}", med);
    println!("Mode: {}", mod_val);
}
