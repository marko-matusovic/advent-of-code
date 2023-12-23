use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::libs::{dir_2d::Dir4, Pos2U};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    start: Pos2U,
    end: Pos2U,
    edges: HashMap<Pos2U, HashSet<Pos2U>>,
    edges_compressed: HashMap<Pos2U, Vec<(Pos2U, usize)>>,
}

fn parse_input(raw: &str) -> Input {
    let mut lines: VecDeque<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    lines.push_front("#".repeat(lines[0].len()));
    lines.push_back("#".repeat(lines[0].len()));
    let lines: Vec<String> = lines.iter().map(|l| l.to_owned()).collect_vec();

    let start = Pos2U(1, 1);
    let end = Pos2U(lines[0].len() - 2, lines.len() - 2);
    let mut edges: HashMap<Pos2U, HashSet<Pos2U>> = HashMap::new();

    for y in 1..lines.len() - 1 {
        for x in 1..lines[y].len() - 1 {
            let cur_pos = Pos2U(x, y);
            let ground = lines[y].chars().nth(x).unwrap();
            match ground {
                '#' => continue,
                '.' => edges.entry(cur_pos).or_insert_with(HashSet::new).extend(
                    Dir4::all().into_iter().filter_map(|d| {
                        let next_pos: Pos2U = (cur_pos + d.dir()).try_into().unwrap();
                        let next_ground = lines[next_pos.1].chars().nth(next_pos.0).unwrap();
                        return match next_ground {
                            '#' => None,
                            _ => Some(next_pos),
                        };
                    }),
                ),
                _ => {
                    let next_dir = match ground {
                        '>' => Dir4::E,
                        '<' => Dir4::W,
                        'v' => Dir4::S,
                        '^' => Dir4::N,
                        _ => panic!(),
                    };
                    edges
                        .entry(cur_pos)
                        .or_insert_with(HashSet::new)
                        .insert((cur_pos + next_dir.dir()).try_into().unwrap());
                }
            }
        }
    }

    let mut edges_compressed = edges
        .iter()
        .map(|(node, neighs)| {
            (
                node.clone(),
                neighs.into_iter().map(|n| (n.clone(), 1)).collect(),
            )
        })
        .collect();

    while compress_some_edge(&mut edges_compressed).is_some() {}

    Input {
        lines,
        start,
        end,
        edges,
        edges_compressed,
    }
}

fn compress_some_edge(edges_compressed: &mut HashMap<Pos2U, Vec<(Pos2U, usize)>>) -> Option<Pos2U> {
    let keys: Vec<Pos2U> = edges_compressed.keys().cloned().collect();
    for edge in keys {
        if edges_compressed[&edge].len() != 2 {
            continue;
        }
        let (neigh0, len0) = edges_compressed[&edge][0];
        let (neigh1, len1) = edges_compressed[&edge][1];
        if let Some(pos) = edges_compressed[&neigh0]
            .iter()
            .position(|&(n, _)| n == edge)
        {
            assert!(len0 == edges_compressed[&neigh0][pos].1);
            let neigh0_list = edges_compressed.get_mut(&neigh0).unwrap();
            neigh0_list.remove(pos);
            neigh0_list.push((neigh1, len0 + len1))
        }
        if let Some(pos) = edges_compressed[&neigh1]
            .iter()
            .position(|&(n, _)| n == edge)
        {
            assert!(len1 == edges_compressed[&neigh1][pos].1);
            let neigh1_list = edges_compressed.get_mut(&neigh1).unwrap();
            neigh1_list.remove(pos);
            neigh1_list.push((neigh0, len0 + len1))
        }
        edges_compressed.remove(&edge);
        return Some(edge.clone());
    }
    None
}

pub struct Day23;
impl Day for Day23 {
    fn day(&self) -> u8 {
        23
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let path = longest_path(
            &input.edges_compressed,
            &input.start,
            &input.end,
            &HashSet::new(),
        );

        println!("Answer is {}", path.unwrap());
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let mut modified = raw.to_owned();
        modified = modified.replace(">", ".");
        modified = modified.replace("<", ".");
        modified = modified.replace("^", ".");
        modified = modified.replace("v", ".");
        let input: Input = parse_input(modified.as_str());

        let path = longest_path(
            &input.edges_compressed,
            &input.start,
            &input.end,
            &HashSet::new(),
        );

        println!("Answer is {}", path.unwrap());
    }
}

fn longest_path(
    edges: &HashMap<Pos2U, Vec<(Pos2U, usize)>>,
    from: &Pos2U,
    to: &Pos2U,
    explored: &HashSet<Pos2U>,
) -> Option<usize> {
    if from == to {
        return Some(0);
    }

    let mut explored = explored.clone();
    explored.insert(from.clone());

    edges
        .get(from)
        .map(|neighbors| {
            neighbors
                .iter()
                .filter(|(neigh, _)| !explored.contains(neigh))
                .filter_map(|(neigh, len)| {
                    longest_path(edges, &neigh, to, &explored).map(|long| long + len)
                })
                .max()
        })
        .flatten()
}
