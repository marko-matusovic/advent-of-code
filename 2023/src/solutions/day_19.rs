use std::{
    cmp::{max, min, Ordering},
    collections::HashMap,
};

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Outcome {
    Accept,
    Reject,
    Forward(String),
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "A" => Outcome::Accept,
            "R" => Outcome::Reject,
            _ => Outcome::Forward(value.to_owned()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Rule {
    Default(Outcome),
    Optional(char, Ordering, usize, Outcome),
}

impl Rule {
    fn evaluate(&self, part: &Part) -> Option<Outcome> {
        match self {
            Rule::Default(o) => Some(o.to_owned()),
            Rule::Optional(v, c, n, o) => {
                if part.get(v).cmp(n) == *c {
                    return Some(o.to_owned());
                }
                None
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> Outcome {
        for rule in self.rules.clone() {
            if let Some(o) = rule.evaluate(part) {
                return o;
            }
        }
        panic!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get(&self, var: &char) -> usize {
        match var {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(),
        }
    }

    fn new(n: usize) -> Part {
        Part {
            x: n,
            m: n,
            a: n,
            s: n,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    lines: Vec<String>,
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Input {
    fn evaluate(&self, part: &Part, workflow: &str) -> Outcome {
        match self.workflows.get(workflow).unwrap().evaluate(part) {
            Outcome::Forward(w2) => self.evaluate(part, w2.as_str()),
            o => o,
        }
    }
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let workflows = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (name, rules) = l.split_once("{").unwrap();
            (
                name.to_owned(),
                Workflow {
                    name: name.to_owned(),
                    rules: rules
                        .strip_suffix("}")
                        .unwrap()
                        .split(",")
                        .map(|rule| {
                            if let Some((cnd, out)) = rule.split_once(":") {
                                let v = cnd.chars().nth(0).unwrap();
                                let c = match cnd.chars().nth(1).unwrap() {
                                    '<' => Ordering::Less,
                                    '>' => Ordering::Greater,
                                    _ => panic!(),
                                };
                                let n = cnd[2..].parse().unwrap();
                                Rule::Optional(v, c, n, Outcome::from(out))
                            } else {
                                Rule::Default(Outcome::from(rule))
                            }
                        })
                        .collect_vec(),
                },
            )
        })
        .collect();
    let parts = lines
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            let vars = l
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(",")
                .map(|n| n.split_once("=").unwrap().1.parse::<usize>().unwrap())
                .collect_vec();
            Part {
                x: vars[0],
                m: vars[1],
                a: vars[2],
                s: vars[3],
            }
        })
        .collect();
    Input {
        lines,
        workflows,
        parts,
    }
}

pub struct Day19;
impl Day for Day19 {
    fn day(&self) -> u8 {
        19
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        // dbg!(&input.workflows);

        let sum: usize = input
            .parts
            .iter()
            .filter(|p| input.evaluate(p, "in") == Outcome::Accept)
            .map(|p| p.x + p.m + p.a + p.s)
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let bounds = accepted_bounds(&input.workflows, "in");

        println!(
            "Check: Answer for P1: {}",
            input
                .parts
                .iter()
                .filter(|p| bounds.iter().any(|bound| bound.0.x <= p.x
                    && p.x <= bound.1.x
                    && bound.0.m <= p.m
                    && p.m <= bound.1.m
                    && bound.0.a <= p.a
                    && p.a <= bound.1.a
                    && bound.0.s <= p.s
                    && p.s <= bound.1.s))
                .map(|p| p.x + p.m + p.a + p.s)
                .sum::<usize>()
        );

        let sum: usize = bounds
            .iter()
            .map(|b| {
                (b.1.x - b.0.x + 1) as usize
                    * (b.1.m - b.0.m + 1) as usize
                    * (b.1.a - b.0.a + 1) as usize
                    * (b.1.s - b.0.s + 1) as usize
            })
            .sum();

        println!("Answer is {}", sum);
    }
}

type Bounds = (Part, Part);

fn accepted_bounds(workflows: &HashMap<String, Workflow>, name: &str) -> Vec<Bounds> {
    let wf = workflows.get(name).unwrap();

    let mut all_bounds: Vec<Bounds> = Vec::new();
    let mut bounds = (Part::new(1), Part::new(4000));
    for rule in wf.rules.iter() {
        match rule.to_owned() {
            Rule::Default(Outcome::Accept) => {
                all_bounds.push(bounds.clone());
            }
            Rule::Default(Outcome::Reject) => {}
            Rule::Default(Outcome::Forward(fwd)) => {
                let sub_bounds = accepted_bounds(workflows, fwd.as_str());
                let sub_modified = sub_bounds
                    .iter()
                    .map(|sb| intersect_bounds(&bounds, sb))
                    .collect_vec();
                all_bounds.extend(sub_modified);
            }
            Rule::Optional(v, c, n, Outcome::Accept) => {
                let modified = modify_bounds(&bounds, &v, &c, &n);
                all_bounds.push(modified);
                bounds = modify_bounds_op(&bounds, &v, &c, &n);
            }
            Rule::Optional(v, c, n, Outcome::Reject) => {
                bounds = modify_bounds_op(&bounds, &v, &c, &n);
            }
            Rule::Optional(v, c, n, Outcome::Forward(fwd)) => {
                let modified = modify_bounds(&bounds, &v, &c, &n);
                let sub_bounds = accepted_bounds(workflows, fwd.as_str());
                let sub_modified = sub_bounds
                    .iter()
                    .map(|sb| intersect_bounds(&modified, sb))
                    .collect_vec();
                all_bounds.extend(sub_modified);
                bounds = modify_bounds_op(&bounds, &v, &c, &n);
            }
        }
    }

    return all_bounds
        .iter()
        .filter(|b| valid_bound(b))
        .map(|b| b.to_owned())
        .collect_vec();
}

fn modify_bounds_op(bounds: &Bounds, v: &char, c: &Ordering, n: &usize) -> Bounds {
    match c {
        Ordering::Less => modify_bounds(bounds, v, &Ordering::Greater, &(*n - 1)),
        Ordering::Greater => modify_bounds(bounds, v, &Ordering::Less, &(*n + 1)),
        _ => panic!(),
    }
}

fn modify_bounds(bounds: &Bounds, v: &char, c: &Ordering, n: &usize) -> Bounds {
    let mut res = bounds.clone();
    match c {
        Ordering::Less => match v {
            'x' => res.1.x = min(res.1.x, *n - 1),
            'm' => res.1.m = min(res.1.m, *n - 1),
            'a' => res.1.a = min(res.1.a, *n - 1),
            's' => res.1.s = min(res.1.s, *n - 1),
            _ => panic!()
        },
        Ordering::Greater => match v {
            'x' => res.0.x = max(res.0.x, *n + 1),
            'm' => res.0.m = max(res.0.m, *n + 1),
            'a' => res.0.a = max(res.0.a, *n + 1),
            's' => res.0.s = max(res.0.s, *n + 1),
            _ => panic!()
        }
        _ => panic!(),
    }

    return res;
}

fn intersect_bounds(bound_a: &Bounds, bound_b: &Bounds) -> Bounds {
    (
        Part {
            x: max(bound_a.0.x, bound_b.0.x),
            m: max(bound_a.0.m, bound_b.0.m),
            a: max(bound_a.0.a, bound_b.0.a),
            s: max(bound_a.0.s, bound_b.0.s),
        },
        Part {
            x: min(bound_a.1.x, bound_b.1.x),
            m: min(bound_a.1.m, bound_b.1.m),
            a: min(bound_a.1.a, bound_b.1.a),
            s: min(bound_a.1.s, bound_b.1.s),
        },
    )
}

fn valid_bound(bound: &Bounds) -> bool {
    bound.0.x <= bound.1.x
        && bound.0.m <= bound.1.m
        && bound.0.a <= bound.1.a
        && bound.0.s <= bound.1.s
}
