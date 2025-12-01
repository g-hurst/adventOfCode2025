use std::fs;

/// Day 01: count how many times the dial points at 0 after applying all rotations.
///
/// - Dial values range from 0 to 99, inclusive.
/// - The dial starts at 50.
/// - Each line of input is of the form `L<number>` or `R<number>`.
/// - `L` means rotate toward lower numbers, `R` toward higher numbers.
/// - The dial wraps around (modulo 100).
/// - After each rotation, if the dial points at 0, increment the counter.
///
/// This is the solution for **part 1**.
pub fn part1(use_example: bool) -> String {
    let day = 1u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };

    let input = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    solve_part1(&input)
}

fn solve_part1(input: &str) -> String {
    let mut position: i32 = 50; // starting position
    let mut count_zero = 0u32;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // First character is direction, rest is distance.
        let (dir, rest) = line.split_at(1);
        let distance: i32 = rest
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Invalid distance in rotation '{}': {}", line, e));

        let steps = distance % 100; // rotating 100 steps is a full circle

        position = match dir {
            "L" => (position - steps).rem_euclid(100),
            "R" => (position + steps).rem_euclid(100),
            _ => panic!("Invalid direction in rotation '{}'", line),
        };

        if position == 0 {
            count_zero += 1;
        }
    }

    count_zero.to_string()
}

/// Day 01, part 2: count how many times any click causes the dial to point at 0,
/// including intermediate positions during rotations.
///
/// That is, for each individual "click" while applying the rotations (including
/// the final click that lands on the end position), count every time the dial
/// is exactly at 0.
pub fn part2(use_example: bool) -> String {
    let day = 1u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };

    let input = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    solve_part2(&input)
}

fn solve_part2(input: &str) -> String {
    let mut position: i32 = 50; // starting position
    let mut count_zero: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (dir, rest) = line.split_at(1);
        let distance: i64 = rest
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("Invalid distance in rotation '{}': {}", line, e));

        if distance < 0 {
            panic!("Negative distances are not supported in rotation '{}'", line);
        }

        let d = distance as u64;
        if d == 0 {
            continue;
        }

        // Count how many intermediate clicks (including the final one) land on 0.
        let hits = match dir {
            "L" => count_hits_left(position, d),
            "R" => count_hits_right(position, d),
            _ => panic!("Invalid direction in rotation '{}'", line),
        };

        count_zero += hits;

        // Update final position (only depends on distance modulo 100).
        let steps = (distance % 100) as i32;
        position = match dir {
            "L" => (position - steps).rem_euclid(100),
            "R" => (position + steps).rem_euclid(100),
            _ => unreachable!("direction already validated"),
        };
    }

    count_zero.to_string()
}

fn count_hits_right(start: i32, distance: u64) -> u64 {
    // Positions visited: (start + k) mod 100 for k in 1..=distance.
    // Solve (start + k) ≡ 0 (mod 100) => k ≡ (100 - start) (mod 100).
    let p = start.rem_euclid(100) as u64;
    let mut base = (100 - p) % 100; // in [0,99]
    if base == 0 {
        base = 100; // first time we hit 0 going right
    }
    if base > distance {
        0
    } else {
        1 + (distance - base) / 100
    }
}

fn count_hits_left(start: i32, distance: u64) -> u64 {
    // Positions visited: (start - k) mod 100 for k in 1..=distance.
    // Solve (start - k) ≡ 0 (mod 100) => k ≡ start (mod 100).
    let p = start.rem_euclid(100) as u64;
    let mut base = p % 100; // in [0,99]
    if base == 0 {
        base = 100; // first time we hit 0 going left
    }
    if base > distance {
        0
    } else {
        1 + (distance - base) / 100
    }
}
