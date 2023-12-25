use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use itertools::Itertools;

use crate::libs::math::lcm_vec;

use super::day_trait::Day;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Signal {
    Low,
    High,
}

impl Signal {
    fn is_high(&self) -> bool {
        *self == Self::High
    }
    fn is_low(&self) -> bool {
        *self == Self::Low
    }
}

#[derive(Clone, Hash)]
struct SignalTransmission {
    from: String,
    to: String,
    signal: Signal,
}

impl Debug for SignalTransmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -{}-> {}",
            self.from,
            match self.signal {
                Signal::Low => "low",
                Signal::High => "high",
            },
            self.to
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, Signal>),
}

impl Hash for ModuleType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ModuleType::Broadcast => {
                "Broadcast".hash(state);
            }
            ModuleType::FlipFlop(is_on) => {
                "FlipFlop".hash(state);
                is_on.hash(state);
            }
            ModuleType::Conjunction(prev) => {
                "Conjunction".hash(state);
                let prev_list: Vec<_> = prev.iter().sorted_by_key(|&(k, _)| k.to_owned()).collect();
                prev_list.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct Module {
    name: String,
    next: Vec<String>,
    typ: ModuleType,
}

impl Module {
    fn process_signal(&mut self, from: &str, signal: &Signal) -> Vec<SignalTransmission> {
        return match &mut self.typ {
            ModuleType::Broadcast => self
                .next
                .iter()
                .map(|nxt| SignalTransmission {
                    from: self.name.to_owned(),
                    to: nxt.to_owned(),
                    signal: signal.to_owned(),
                })
                .collect_vec(),
            ModuleType::FlipFlop(ref mut is_on) => {
                if signal.is_low() {
                    *is_on = *is_on ^ true;
                    return self
                        .next
                        .iter()
                        .map(|nxt| SignalTransmission {
                            from: self.name.to_owned(),
                            to: nxt.to_owned(),
                            signal: match is_on {
                                false => Signal::Low,
                                true => Signal::High,
                            },
                        })
                        .collect_vec();
                }
                return Vec::new();
            }
            ModuleType::Conjunction(ref mut prev) => {
                prev.insert(from.to_owned(), signal.to_owned());
                let send = match prev.values().all(|v| v.is_high()) {
                    true => Signal::Low,
                    false => Signal::High,
                };
                return self
                    .next
                    .iter()
                    .map(|nxt| SignalTransmission {
                        from: self.name.to_owned(),
                        to: nxt.to_owned(),
                        signal: send.clone(),
                    })
                    .collect_vec();
            }
        };
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    lines: Vec<String>,
    modules: HashMap<String, Module>,
    back_links: HashMap<String, Vec<String>>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let mut modules: HashMap<String, Module> = lines
        .iter()
        .map(|l| {
            let pat: &[_] = &['&', '%'];
            let name = l
                .split_once(" ")
                .unwrap()
                .0
                .trim_start_matches(pat)
                .to_owned();
            let next = l
                .split_once(" -> ")
                .unwrap()
                .1
                .split(", ")
                .map(|s| s.to_owned())
                .collect_vec();
            (
                name.to_owned(),
                Module {
                    name,
                    next,
                    typ: match l.chars().nth(0) {
                        Some('b') => ModuleType::Broadcast,
                        Some('%') => ModuleType::FlipFlop(false),
                        Some('&') => ModuleType::Conjunction(HashMap::new()),
                        _ => panic!(),
                    },
                },
            )
        })
        .collect();

    let mut back_links: HashMap<String, Vec<String>> = HashMap::new();
    for (_, module) in modules.iter() {
        for next in module.next.iter() {
            if !back_links.contains_key(next) {
                back_links.insert(next.to_owned(), Vec::new());
            }
            back_links
                .get_mut(next)
                .unwrap()
                .push(module.name.to_owned())
        }
    }

    for (_, module) in modules.iter_mut() {
        match &mut module.typ {
            ModuleType::Conjunction(ref mut prev) => {
                if let Some(link) = back_links.get(&module.name) {
                    prev.extend(link.iter().map(|s| (s.to_owned(), Signal::Low)));
                }
            }
            _ => {}
        }
    }

    Input {
        lines,
        modules,
        back_links,
    }
}

pub struct Day20;
impl Day for Day20 {
    fn day(&self) -> u8 {
        20
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);
        let mut modules = input.modules.clone();

        let mut count_low = 0;
        let mut count_high = 0;
        for _ in 0..1000 {
            let (low, high) = press_button_n_count_signals(&mut modules);
            count_low += low;
            count_high += high;
        }

        println!("Answer is {}", count_low * count_high);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let prev1 = &input.back_links.get("rx").unwrap()[0];
        assert!(matches!(
            input.modules.get(prev1).unwrap().typ,
            ModuleType::Conjunction(_)
        ));
        let prev2 = input.back_links.get(prev1).unwrap().to_owned();
        for hm in prev2.iter() {
            assert!(matches!(
                input.modules.get(hm).unwrap().typ,
                ModuleType::Conjunction(_)
            ))
        }
        let prev3: Vec<String> = prev2
            .iter()
            .map(|m| {
                let bm = input.back_links.get(m).unwrap();
                assert!(bm.len() == 1);
                return bm[0].to_owned();
            })
            .collect();
        for hm in prev3.iter() {
            assert!(matches!(
                input.modules.get(hm).unwrap().typ,
                ModuleType::Conjunction(_)
            ))
        }

        dbg!(&prev3);

        let mut modules = input.modules.clone();
        let mut cycles: HashMap<String, usize> = prev2.iter().map(|p| (p.to_owned(), 0)).collect();
        let mut i = 0;
        while cycles.iter().any(|(_, c)| *c == 0) {
            let transmissions = press_button_n_record_transmissions(&mut modules);
            i += 1;
            print!("{}\r", &i);
            for (md, c) in cycles.iter_mut() {
                if *c != 0 {
                    continue;
                }
                if transmissions.iter().any(|st| st.from == *md && st.signal.is_high()) {
                    *c = i;
                    println!("{}: {}", &md, &c);
                }
            }
        }

        let first = lcm_vec(&cycles.values().cloned().collect_vec());

        println!("Answer is {}", first);
    }
}

fn press_button_n_count_signals(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut signals: Vec<SignalTransmission> = vec![SignalTransmission {
        from: String::from("button"),
        to: String::from("broadcaster"),
        signal: Signal::Low,
    }];

    let mut count_low = 1;
    let mut count_high = 0;

    while !signals.is_empty() {
        signals = signals
            .iter()
            .flat_map(|st| {
                modules
                    .get_mut(&st.to.to_owned())
                    .map(|module| module.process_signal(&st.from, &st.signal))
            })
            .flatten()
            .collect_vec();
        count_low += signals.iter().filter(|t| t.signal.is_low()).count();
        count_high += signals.iter().filter(|t| t.signal.is_high()).count();
    }

    return (count_low, count_high);
}

fn press_button_n_record_transmissions(modules: &mut HashMap<String, Module>) -> Vec<SignalTransmission> {
    let mut signals: Vec<SignalTransmission> = vec![SignalTransmission {
        from: String::from("button"),
        to: String::from("broadcaster"),
        signal: Signal::Low,
    }];

    let mut transmissions: Vec<SignalTransmission> = Vec::new();
    transmissions.extend(signals.clone());

    while !signals.is_empty() {
        signals = signals
            .iter()
            .flat_map(|st| {
                modules
                    .get_mut(&st.to.to_owned())
                    .map(|module| module.process_signal(&st.from, &st.signal))
            })
            .flatten()
            .collect_vec();
        transmissions.extend(signals.clone());
    }

    return transmissions;
}
