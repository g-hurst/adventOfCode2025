use std::fs;
use std::collections::{HashSet, VecDeque};


type Point = (u64, u64);

fn parse_splitters(use_example: bool) -> (Point, HashSet<Point>) {
    let day = 7u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };

    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    let mut points = HashSet::new();
    let mut start = (0, 0);

    for (y, line) in text.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => { points.insert((x as u64, y as u64)); }
                'S' => { start = (x as u64, y as u64); }
                _ => {}
            }
        }
    }

    (start, points)
}

fn count_splits(start: Point, splitters: &HashSet<Point>) -> u64 {
    let max_y = splitters.iter().map(|(_, y)| *y).max().unwrap_or(0);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();
    let mut total_splits = 0u64;
    
    queue.push_back(start);
    while let Some((x, mut y)) = queue.pop_front() {
        while y <= max_y {
            if visited.contains(&(x, y)) {
                break;
            }
            visited.insert((x, y));
            if splitters.contains(&(x, y)) {
                total_splits += 1;
                queue.push_back((x - 1, y));
                queue.push_back((x + 1, y));
                break;
            }
            y += 1;
        }
    }

    total_splits
}

fn count_timelines(start: Point, splitters: &HashSet<Point>) -> u64 {
    let mut splitters: Vec<(usize, usize)> = splitters
        .iter()
        .cloned()
        .map(|(x, y)| (x as usize, y as usize))
        .collect();
    splitters.sort_by_key(|(_, y)| *y);

    let max_x = splitters.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let mut beam_counts = vec![0u64; max_x + 2];

    beam_counts[start.0 as usize] = 1;

    for (x, _) in splitters.iter() {
        let count = beam_counts[*x];
        if count > 0 {
            beam_counts[*x - 1] += count;
            beam_counts[*x + 1] += count;
            beam_counts[*x] = 0;
        }
    }

    beam_counts.iter().sum::<u64>()
}



pub fn part1(use_example: bool) -> String {  
    let (start, splitters) = parse_splitters(use_example);
    count_splits(start, &splitters).to_string()
}

pub fn part2(use_example: bool) -> String {
    let (start, splitters) = parse_splitters(use_example);
    count_timelines(start, &splitters).to_string()
}
