use std::fs;

fn parse_equasions(use_example: bool, part: u8) -> Vec<(char, Vec<u64>)> {
    let day = 6u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };



    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));
    let data = text.split('\n').collect::<Vec<&str>>();

    
    if part == 1 {
        let mut equastions: Vec<(char, Vec<u64>)> = vec![(' ', Vec::new()); data[0].split_whitespace().count()];
        for (n, line) in data.iter().enumerate() {
            if line.is_empty() {
                continue;
            }
            for (i, c) in line.split_whitespace().enumerate() {
                if n == data.len() - 1 {
                    let op = c.chars().next().unwrap();
                    equastions[i].0 = op;
                } else {
                    let val: u64 = c.trim().parse().unwrap();
                    equastions[i].1.push(val);
                }
            }
        }

        equastions
    } else {
        let mut nums = vec![0u64; data[0].chars().count()];
        for line in data.iter().take(data.len() - 1) {
            if line.is_empty() {
                continue;
            }
            for (i, c) in line.chars().enumerate() {
                if let Some(val) = c.to_digit(10) {
                    nums[i] = nums[i] * 10 + val as u64;
                }
            }
        }
        
        let mut equastions: Vec<(char, Vec<u64>)> = Vec::new();
        for (i, c) in data[data.len() - 1].chars().enumerate() {
            if c == '+' || c == '*' {
                equastions.insert(0, (c, Vec::new()));
            }
            if nums[i] != 0 {
                equastions[0].1.push(nums[i]);
            }
        }
        
        equastions
    }
}

fn calculate_equasion(op: char, vals: &Vec<u64>) -> u64 {
    match op {
        '+' => vals.iter().sum(),
        '*' => vals.iter().product(),
        _ => panic!("Unknown operation: {}", op),
    }
}

pub fn part1(use_example: bool) -> String {  
    parse_equasions(use_example, 1)
        .iter()
        .map(|(op, vals)| calculate_equasion(*op, vals))
        .sum::<u64>()
        .to_string()
}

pub fn part2(use_example: bool) -> String {
    parse_equasions(use_example, 2)
        .iter()
        .map(|(op, vals)| calculate_equasion(*op, vals))
        .sum::<u64>()
        .to_string()
}
