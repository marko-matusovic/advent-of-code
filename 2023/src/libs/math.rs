pub fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

pub fn gcd_vec(numbers: &[usize]) -> usize {
    numbers.iter().fold(numbers[0], |a, &b| gcd(a, b))
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn lcm_vec(numbers: &[usize]) -> usize {
    numbers.iter().fold(1, |a, &b| lcm(a, b))
}