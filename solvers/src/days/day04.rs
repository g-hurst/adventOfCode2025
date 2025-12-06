use std::fs;
use std::collections::{
    HashMap, 
    HashSet
};

fn parse_map(use_example: bool) -> HashMap<(i32, i32), char> {
    let day = 4u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let data = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    for (x, line) in data.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        for (y, ch) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), ch);
        }
    }

    map
}

pub fn part1(use_example: bool) -> String {
    let map = parse_map(use_example);

    let directions = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
    let is_reachable = |node: (i32, i32)| -> bool {
        let n_neighbors = directions.iter()
            .filter(|(dx, dy)| {
                let neighbor = (node.0 + dx, node.1 + dy);
                matches!(map.get(&neighbor), Some(&'@'))
            })
            .count();
        
        n_neighbors < 4
    };

    let mut sum: u64 = 0;
    for (&node, &ch) in map.iter() {
        if ch == '@' && is_reachable(node) {
            sum += 1;
        }
    }

    sum.to_string()
}

pub fn part2(use_example: bool) -> String {
    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    let mut ats: HashSet<(i32, i32)> = HashSet::new();

    for (key, &value) in parse_map(use_example).iter() {
        match value {
            '.' => { dots.insert(*key); },
            '@' => { ats.insert(*key); },
            _ => {} 
        }
    }
    
    let directions = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
    let is_reachable = |node: (i32, i32), ats: &HashSet<(i32, i32)>| -> bool {
        let n_neighbors = directions.iter()
            .filter(|(dx, dy)| {
                let neighbor = (node.0 + dx, node.1 + dy);
                ats.contains(&neighbor)
            })
            .count();

        n_neighbors < 4
    };

    let mut sum: u64 = 0;
    let mut one_reached = true;
    while one_reached {
        one_reached = false;
        let to_remove: Vec<_> = ats.iter()
            .cloned()
            .filter(|&at| is_reachable(at, &ats))
            .collect();

        for at in to_remove {
            ats.remove(&at);
            dots.insert(at);
            sum += 1;
            one_reached = true;
        }
    }

    sum.to_string()
}
