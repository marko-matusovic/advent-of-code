pub fn day() -> u8 {
    2
}

#[derive(Debug)]
pub struct Input {
    nums: Vec<usize>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<&str> = raw.split(",").collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    Input {
        nums: lines.iter().map(|&l| l.parse().unwrap()).collect(),
    }
}

fn exec(nums: &mut Vec<usize>, i: usize, foo: &dyn Fn(usize, usize) -> usize) {
    let a = nums[i + 1];
    let b = nums[i + 2];
    let c = nums[i + 3];
    nums[c]= foo(nums[a], nums[b]);
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());
    let mut nums = input.nums.clone();
    nums[1] = 12;
    nums[2] = 2;

    let mut i = 0;
    loop {
        match nums[i] {
            1 => exec(&mut nums, i, &|a,b| a+b),
            2 => exec(&mut nums, i, &|a,b| a*b),
            99 => break,
            _ => panic!("bad op call"),
        }
        i += 4;
    }

    println!("Answer is {}", nums[0]);

    // 337076 is too low

}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut res: (usize, usize) = (0,0);
    'out: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut nums = input.nums.clone();
            nums[1] = noun;
            nums[2] = verb;

            let mut i = 0;
            loop {
                match nums[i] {
                    1 => exec(&mut nums, i, &|a,b| a+b),
                    2 => exec(&mut nums, i, &|a,b| a*b),
                    99 => break,
                    _ => panic!("bad op call"),
                }
                i += 4;
            }

            if nums[0] == 19690720 {
                res = (noun, verb);
                break 'out;
            }
        }

    }

    println!("Answer is {}", res.0 * 100 + res.1);
}
