use std::fs;

fn parse_ranges(use_example: bool) -> Vec<(u64, u64)> {
    let day = 2u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };

    let input = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    // Remove all whitespace (including newlines) and split by commas
    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    
    cleaned
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range_str| {
            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() != 2 {
                panic!("Invalid range format: '{}'", range_str);
            }
            let start: u64 = parts[0]
                .parse()
                .unwrap_or_else(|e| panic!("Invalid start in range '{}': {}", range_str, e));
            let end: u64 = parts[1]
                .parse()
                .unwrap_or_else(|e| panic!("Invalid end in range '{}': {}", range_str, e));
            (start, end)
        })
        .collect()
}

fn next_invalid_id(start: u64, max: u64, part: u8) -> Option<u64> {
    fn is_invalid_id_part1(n: u64) -> bool {
        let s = n.to_string();
        let len = s.len();
        if len % 2 != 0 {
            return false;
        }
        let half = len / 2;
        &s[0..half] == &s[half..]
    }

    fn is_invalid_id_part2(n: u64) -> bool {
        let s = n.to_string();
        let len = s.len();
        // Try all possible segment lengths from 1 to len/2
        // For each segment length, check if the number can be divided into
        // equal segments that all match (at least 2 repetitions)
        for segment_len in 1..=(len / 2) {
            // Check if len is divisible by segment_len (so we can have equal segments)
            if len % segment_len != 0 {
                continue;
            }
            let num_repetitions = len / segment_len;
            // Must have at least 2 repetitions
            if num_repetitions < 2 {
                continue;
            }
            // Extract the first segment
            let first_segment = &s[0..segment_len];   
            // Check if all segments match the first segment
            let mut all_match = true;
            for i in 1..num_repetitions {
                let segment_start = i * segment_len;
                let segment_end = segment_start + segment_len;
                if &s[segment_start..segment_end] != first_segment {
                    all_match = false;
                    break;
                }
            }
            if all_match {
                return true;
            }
        }
        false
    }
    
    let is_invalid_id = match part {
        1 => is_invalid_id_part1,
        2 => is_invalid_id_part2,
        _ => panic!("Invalid part: {}", part),
    };

    for i in start..(max+1) {
        if is_invalid_id(i) {
            return Some(i);
        }
    }
    return None;
}

fn sum_invalid_ids(start: u64, end: u64, part: u8) -> u64 {
    let mut sum: u64 = 0;
    let mut current_id: u64 = start;
    while let Some(bad_id) = next_invalid_id(current_id, end, part) {
        sum += bad_id;
        current_id = bad_id + 1;
    }
    return sum;
}

pub fn part1(use_example: bool) -> String {
    let mut sum: u64 = 0;
    for range in parse_ranges(use_example).iter() {
        sum += sum_invalid_ids(range.0, range.1, 1);
    }
    sum.to_string()
}

pub fn part2(use_example: bool) -> String {
    let mut sum: u64 = 0;
    for range in parse_ranges(use_example).iter() {
        sum += sum_invalid_ids(range.0, range.1, 2);
    }
    sum.to_string()
}
