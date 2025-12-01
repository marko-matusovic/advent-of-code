pub trait Day {
    fn day(&self) -> u8;
    fn part_1(&self, input: &str);
    fn part_2(&self, input: &str);
}
