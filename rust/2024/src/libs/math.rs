pub fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

pub fn gcd_vec(numbers: &Vec<usize>) -> usize {
    numbers.iter().fold(numbers[0], |a, &b| gcd(a, b))
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn lcm_vec(numbers: &Vec<usize>) -> usize {
    numbers.iter().fold(1, |a, &b| lcm(a, b))
}

pub fn divides(a: usize, b: usize) -> bool {
    (a as f64 / b as f64) % 1. == 0.
}

pub fn solve_linear_system(a: &[[f64; 2]; 2], y: &[f64; 2]) -> Option<[f64; 2]> {
    let mut a = a.clone();
    let mut y = y.clone();

    if a[0][0] == 0. {
        return None;
    }
    // norm top row
    a[0][1] /= a[0][0];
    y[0] /= a[0][0];
    a[0][0] = 1.;

    // eliminate x[0] from btm row
    a[1][1] -= a[0][1] * a[1][0];
    y[1] -= y[0] * a[1][0];
    a[1][0] = 0.;

    // norm btm row
    if a[1][1] == 0. {
        return None;
    }
    y[1] /= a[1][1];
    a[1][1] = 1.;

    // eliminate x[1] from top row
    y[0] -= y[1] * a[0][1];
    a[0][1] = 0.;

    let round_y0: f64 = (y[0] * 1000.).round() / 1000.;
    let round_y1: f64 = (y[1] * 1000.).round() / 1000.;

    Some([round_y0, round_y1])
}

pub fn is_whole(n: f64) -> bool {
    let off = n % 1.;
    let min_off = if off < 0.5 { off } else { 1. - off };

    min_off < 0.001
}
