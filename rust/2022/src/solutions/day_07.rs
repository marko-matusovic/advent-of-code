use std::collections::HashMap;

pub fn day() -> u8 {
    7
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum FileSys {
    Directory(String, Vec<FileSys>),
    File(String, u64),
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    Input { lines: lines }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut it = input.lines.iter();
    it.next(); // $ cd /
    let root: FileSys = parse_directory("", &mut it);

    let mut sizes: HashMap<&str, u64> = HashMap::new();
    sum_dir(&root, &mut sizes);

    let total: u64 = sizes.values().filter(|&&v| v <= 100_000_u64).sum();

    println!("Answer is {}", total);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());
    
    let total_size: u64 = 70000000_u64;
    let required: u64 = 30000000_u64;

    let mut it = input.lines.iter();
    it.next(); // $ cd /
    let root: FileSys = parse_directory("", &mut it);

    let mut sizes: HashMap<&str, u64> = HashMap::new();
    let used:u64 = sum_dir(&root, &mut sizes);

    let free_up = required - (total_size - used);
    let res: &u64 = sizes.values().filter(|&&v| v >= free_up).min().unwrap();

    println!("Answer is {}", res);
}

fn sum_dir<'a>(dir: &'a FileSys, sizes: &mut HashMap<&'a str, u64>) -> u64 {
    let size = match dir {
        FileSys::File(_, size) => size.clone(),
        FileSys::Directory(_, sub_dirs) => sub_dirs.iter().map(|d| sum_dir(d, sizes)).sum()
    };
    if let FileSys::Directory(name, _) = dir {
        sizes.insert(name, size.clone());
    }
    return size;
}

fn parse_directory(name: &str, it: &mut std::slice::Iter<String>) -> FileSys {
    let mut content: Vec<FileSys> = Vec::new();

    let mut line: &str = it.next().unwrap();
    if line.eq("$ ls") {
        loop {
            if let Some(v) = it.next() {
                line = v;
                if line.starts_with("$") {
                    break;
                }
                let (first, second) = line.split_once(' ').unwrap();
                if first.eq("dir") {
                    continue;
                }
                content.push(FileSys::File(format!("{}/{}", name, second.to_owned()), first.parse().unwrap()));
            } else {
                return FileSys::Directory(name.to_owned(), content);
            }
        }
    }
    loop {
        if line.eq("$ cd ..") {
            break;
        } else if line.starts_with("$ cd") {
            content.push(parse_directory(&format!("{}/{}", name, line.split_at(5).1), it));
            match it.next() {
                Some(v) => line = v,
                None => break,
            }
        } else {
            panic!("unexpected input {}", line);
        }
    }

    return FileSys::Directory(name.to_owned(), content);
}
