use std::fs;

fn parse_products(use_example: bool) -> (Vec<u64>, Vec<(u64, u64)>) {
    let day = 5u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };

    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    let data = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));
    for line in data.split('\n') {
        if line.is_empty() {
            continue;
        }
        if line.contains('-') {
            fresh_ranges.push({
                let parts: Vec<&str> = line.split('-').collect();
                let start: u64 = parts[0].trim().parse().unwrap();
                let end: u64 = parts[1].trim().parse().unwrap();
                (start, end)
            });
        } else {
            ingredients.push(line.trim().parse().unwrap());
        }
    }

    (ingredients, fresh_ranges)
}

pub fn part1(use_example: bool) -> String {
    let (ingredients, fresh_ranges) = parse_products(use_example);
    let is_fresh = |ingredient: u64| -> bool {
        for (start, end) in fresh_ranges.iter() {
            if ingredient >= *start && ingredient <= *end {
                return true;
            }
        }
        false
    };
    
    let mut sum: u64 = 0;
    for ingredient in ingredients.iter() {
        if is_fresh(*ingredient) {
            sum += 1;
        }
    }

    sum.to_string()
}

pub fn part2(use_example: bool) -> String {
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for range in parse_products(use_example).1.iter() {
        let mut new_range = *range;
        merged_ranges.retain(|m_range| {
            if new_range.0 <= m_range.1 && m_range.0 <= new_range.1 {
                // remove old range
                new_range.0 = new_range.0.min(m_range.0);
                new_range.1 = new_range.1.max(m_range.1);
                false 
            } else {
                // keep non-overlapping ranges
                true 
            }
        });
        merged_ranges.push(new_range);
    }

    let mut sum: u64 = 0;
    for (left, right) in merged_ranges.iter() {
        sum += right - left + 1;
    }
    sum.to_string()
}
