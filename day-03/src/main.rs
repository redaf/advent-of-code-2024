fn main() {
    println!("Hello, world!");
}

fn part_1_sum_of_uncorrupted_muls(memory: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part_1_example_result() {
        assert_eq!(161, part_1_sum_of_uncorrupted_muls(EXAMPLE_PART_1));
    }
}
