use std::fs;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> u64 {
            self.x.abs_diff(other.x).pow(2) + 
            self.y.abs_diff(other.y).pow(2) + 
            self.z.abs_diff(other.z).pow(2)
    }
}

fn parse_points(use_example: bool) -> Vec<Point> {
    let day = 8u8;
    let path = if use_example {
        format!("inputs/day{day:02}_example.txt")
    } else {
        format!("inputs/day{day:02}.txt")
    };

    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    let mut points = Vec::new();

    for line in text.lines() {
        let coords: Vec<u64> = line
            .split(',')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();
        if coords.len() != 3 {
            panic!("Invalid point format: {}", line);
        }
        points.push(Point::new(coords[0], coords[1], coords[2]));
    }

    points
}

fn make_min_circuits(points: &Vec<Point>, n_connections: u64) -> u64{
    // get the smallest distances
    let mut queue = BinaryHeap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() { 
            let dist = points[i].distance(&points[j]);
            queue.push(Reverse((dist, (i, j))));
        }
    }

    // create clusters of connected points
    let mut clusters: Vec<HashSet<usize>> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    for _ in 0..n_connections {
        let Some(Reverse((_, (i, j)))) = queue.pop() else {
            break;
        };
        if !seen.contains(&i) && !seen.contains(&j) {
            // new cluster
            let mut new_cluster = HashSet::new();
            new_cluster.insert(i);
            new_cluster.insert(j);
            clusters.push(new_cluster);
            seen.insert(i);
            seen.insert(j);
        } else if seen.contains(&i) && !seen.contains(&j) {
            // add j to i's cluster
            for cluster in clusters.iter_mut() {
                if cluster.contains(&i) {
                    cluster.insert(j);
                    seen.insert(j);
                    break;
                }
            }
        } else if !seen.contains(&i) && seen.contains(&j) {
            // add i to j's cluster
            for cluster in clusters.iter_mut() {
                if cluster.contains(&j) {
                    cluster.insert(i);
                    seen.insert(i);
                    break;
                }
            }
        } else {
            // both are already in clusters, possibly the same one
            let mut i_cluster_index: Option<usize> = None;
            let mut j_cluster_index: Option<usize> = None;
            for (index, cluster) in clusters.iter().enumerate() {
                if cluster.contains(&i) {
                    i_cluster_index = Some(index);
                }
                if cluster.contains(&j) {
                    j_cluster_index = Some(index);
                }
            }
            if let (Some(i_idx), Some(j_idx)) = (i_cluster_index, j_cluster_index) {
                if i_idx != j_idx {
                    // merge clusters
                    let points_to_add: Vec<usize> = clusters[j_idx].iter().copied().collect();
                    clusters[i_idx].extend(points_to_add);
                    clusters.remove(j_idx);
                }
            }
        }
    }
    
    clusters.sort_by_key(|c| c.len());
    
    let mut product: u64 = 1;
    for cluster in clusters.iter().rev().take(3) {
        product *= cluster.len() as u64;
    }

    product
}

fn make_mst(points: &Vec<Point>) -> u64 {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut min_heap = BinaryHeap::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() { 
            let dist = points[i].distance(&points[j]);
            min_heap.push(Reverse((dist, (i, j))));
        }
    }

    let mut last_two_product = 1;
    while visited.len() < points.len() {
        let Some(Reverse((_, (u, v)))) = min_heap.pop() else {
            break;
        };

        visited.insert(v);
        visited.insert(u);
        if visited.len() == points.len()  {
            last_two_product = points[u].x * points[v].x;
        }
    }

    last_two_product
}

pub fn part1(use_example: bool) -> String {  
    let points = parse_points(use_example);
    let n_connections = match use_example {
        true => 10,
        false => 1000,
    };
    make_min_circuits(&points, n_connections).to_string()
}

pub fn part2(use_example: bool) -> String {
    let points = parse_points(use_example);
    make_mst(&points).to_string()}
