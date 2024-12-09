use std::usize;

use super::day_trait::Day;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Block {
    File { len: usize, id: usize },
    Gap { len: usize },
}

pub struct Day09;
impl Day for Day09 {
    fn day(&self) -> u8 {
        09
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let mut mem = Vec::new();
        for i in 0..raw.len() {
            let n: usize = raw.chars().nth(i).unwrap() as usize - '0' as usize;
            if n == 0 {
                continue;
            }
            for _ in 0..n {
                mem.push(match i % 2 {
                    0 => Some(i / 2),
                    _ => None,
                });
            }
        }

        let mut i = 0;
        let mut checksum: u128 = 0;
        while i < mem.len() {
            // println!(
            //     "{}",
            //     mem.iter()
            //         .map(|b| match b {
            //             Some(id) => id.to_string(),
            //             None => ".".to_string(),
            //         })
            //         .join("")
            // );
            if mem[i] == None {
                mem[i] = mem.pop().unwrap();
            }
            if let Some(&Some(id)) = mem.get(i) {
                checksum += i as u128 * id as u128;
                i += 1;
            }
        }

        println!("Answer is {}", checksum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let mut mem: Vec<Block> = Vec::new();
        for i in 0..raw.len() {
            let n: usize = raw.chars().nth(i).unwrap() as usize - '0' as usize;
            if n == 0 {
                continue;
            }
            mem.push(match i % 2 {
                0 => Block::File { len: n, id: i / 2 },
                _ => Block::Gap { len: n },
            });
        }

        let files: Vec<Block> = mem
            .iter()
            .filter(|&b| matches!(b, Block::File { len: _, id: _ }))
            .rev()
            .map(|f| f.to_owned())
            .collect();
        for file in files {
            if let Block::File { len, id: _ } = file {
                // println!("no considering file: {}", id);
                // print_mem(&mem);

                let i = mem.iter().position(|&b| b == file).unwrap();
                for j in 0..i {
                    if let Some(&Block::Gap { len: len_gap }) = mem.get(j) {
                        if len <= len_gap {
                            assert!(j < i);
                            mem.remove(i);
                            mem.insert(i, Block::Gap { len: len });
                            mem.remove(j);
                            mem.insert(j, file.to_owned());
                            mem.insert(j + 1, Block::Gap { len: len_gap - len });
                            // println!("moved");
                            // print_mem(&mem);
                            break;
                        }
                    }
                }
            }
        }

        let mut i = 0;
        let mut checksum: u128 = 0;
        for block in mem {
            match block {
                Block::File { len, id } => {
                    for j in i..i + len {
                        checksum += j as u128 * id as u128;
                    }
                    i += len;
                }
                Block::Gap { len } => {
                    i += len;
                }
            }
        }

        println!("Answer is {}", checksum);
    }
}

// fn print_mem(mem: &Vec<Block>) {
//     for block in mem {
//         match block {
//             Block::File { len, id } => {
//                 print!("{}", id.to_string().repeat(len.to_owned()))
//             }
//             Block::Gap { len } => {
//                 print!("{}", ".".repeat(len.to_owned()))
//             }
//         }
//     }
//     println!()
// }
