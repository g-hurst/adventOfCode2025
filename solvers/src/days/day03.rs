use std::fs;

fn parse_batteries(use_example: bool) -> Vec<String> {
    let day = 3u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };

    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn calc_joltage(battery: &str, n: usize) -> u64 {
    let digits: Vec<u8> = battery
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    
    let mut joltage: Vec<u8> = Vec::new();
    let mut last_pos: i32 = -1;
    
    // Find n highest digits in order
    for i in 0..n {
        let mut max_digit = 0u8;
        let mut max_pos = 0usize;
        
        // Search range: from (last_pos + 1) to (len - (n - i - 1))
        // This ensures we leave enough digits for the remaining positions
        let search_start = (last_pos + 1) as usize;
        let search_end = digits.len() - (n - i - 1);
        for pos in search_start..search_end {
            if digits[pos] > max_digit {
                max_digit = digits[pos];
                max_pos = pos;
            }
        }
        
        joltage.push(max_digit);
        last_pos = max_pos as i32;
    }
    
    // Construct the n-digit number from the selected digits
    let mut result = 0u64;
    for digit in joltage.iter() {
        result = result * 10 + (*digit as u64);
    }
    
    result
}

pub fn part1(use_example: bool) -> String {
    let mut sum: u64 = 0;
    for battery in parse_batteries(use_example).iter() {
        sum += calc_joltage(battery, 2);
    }
    sum.to_string()
}

pub fn part2(use_example: bool) -> String {
    let mut sum: u64 = 0;
    for battery in parse_batteries(use_example).iter() {
        sum += calc_joltage(battery, 12);
    }
    sum.to_string()
}
