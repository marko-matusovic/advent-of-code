use std::{
    cmp::{max, min},
    collections::HashSet,
};

use regex::Regex;

pub fn day() -> u8 {
    15
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Range {
    from: isize,
    to: isize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Input {
    sensors: Vec<(Point, Point)>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    let sensors: Vec<(Point, Point)> = lines
        .iter()
        .map(|l| {
            // "Sensor at x=3088287, y=2966967: closest beacon is at x=3340990, y=2451747"
            let reg = Regex::new(r"-?\d+").unwrap();
            let mut digs = reg.find_iter(l);
            let x1: isize = digs.next().unwrap().as_str().parse().unwrap();
            let y1: isize = digs.next().unwrap().as_str().parse().unwrap();
            let x2: isize = digs.next().unwrap().as_str().parse().unwrap();
            let y2: isize = digs.next().unwrap().as_str().parse().unwrap();
            return (Point { x: x1, y: y1 }, Point { x: x2, y: y2 });
        })
        .collect();
    Input { sensors }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let row = 2000000;

    // Find bounds
    let mut bounds: HashSet<Range> = HashSet::new();
    for (s, b) in input.sensors.clone() {
        let dist = dist(&s, &b);
        let half_width = dist - (s.y - row).abs();
        if half_width < 0 {
            continue;
        }
        bounds.insert(Range {
            from: s.x - half_width,
            to: s.x + half_width,
        });
    }

    // Simplify bounds
    loop {
        bounds = bounds
            .iter()
            .map(|&a| match bounds.iter().find(|b| bounds_cross(&a, b)) {
                Some(b) => simplify_bound(&a, b),
                None => a,
            })
            .collect();
        if exclusive(&bounds) {
            break;
        }
    }

    let mut occupied: HashSet<Point> = HashSet::new();
    input.sensors.iter().for_each(|(s, b)| {
        occupied.insert(s.clone());
        occupied.insert(b.clone());
    });

    let mut total: isize = bounds.iter().map(|b| b.to - b.from + 1).sum();

    total -= occupied
        .iter()
        .filter(|p| p.y == row)
        .filter(|p| bounds.iter().any(|b| b.from <= p.x && p.x <= b.to))
        .count() as isize;

    println!("Answer is {}", total);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut sensors: HashSet<Point> = HashSet::new();
    let mut beacons: HashSet<Point> = HashSet::new();
    let mut areas: Vec<(Point, isize)> = Vec::new();
    input.sensors.iter().for_each(|(s, b)| {
        sensors.insert(s.clone());
        beacons.insert(b.clone());
        areas.push((s.clone(), dist(s, b)));
    });

    let mut borders: HashSet<Point> = HashSet::new();
    for (s, b) in input.sensors.iter() {
        let d = dist(s, b) + 1;
        for i in 0..d {
            borders.insert(Point {
                x: s.x + i,
                y: s.y + d - i,
            });
            borders.insert(Point {
                x: s.x + d - i,
                y: s.y - i,
            });
            borders.insert(Point {
                x: s.x - i,
                y: s.y - d + i,
            });
            borders.insert(Point {
                x: s.x - d + i,
                y: s.y + i,
            });
        }
    }

    let limit = 4_000_000;

    let res = borders
        .iter()
        .find(|p| {
            if !(0 <= p.x && p.x <= limit && 0 <= p.y && p.y <= limit) {
                return false;
            }
            if sensors.contains(p) || beacons.contains(p) {
                return false;
            }
            if areas.iter().any(|(s, d)| dist(s, &p) <= *d) {
                return false;
            }
            return true;
        })
        .unwrap();

    let score = 4_000_000 * res.x + res.y;

    println!("Answer is {}", score);
}

fn dist(a: &Point, b: &Point) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn bounds_cross(a: &Range, b: &Range) -> bool {
    (a.from <= b.from && b.from <= a.to)
        || (a.from <= b.to && b.to <= a.to)
        || (b.from <= a.from && a.from <= b.to)
        || (b.from <= a.to && a.to <= b.to)
}

fn simplify_bound(a: &Range, b: &Range) -> Range {
    Range {
        from: min(a.from, b.from),
        to: max(a.to, b.to),
    }
}

fn exclusive(bounds: &HashSet<Range>) -> bool {
    !bounds
        .iter()
        .any(|a| bounds.iter().any(|b| !a.eq(b) && bounds_cross(a, b)))
}
