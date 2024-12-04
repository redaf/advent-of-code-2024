fn main() {
    println!("Hello, world!");
}

fn part_1_sum_of_uncorrupted_muls(memory: &str) -> i32 {
    memory
        .split("mul(")
        .filter(|ins| ins.contains(",") && ins.contains(")"))
        .filter_map(|ins| match ins.find(')') {
            Some(i) => Some(ins.split_at(i).0),
            None => None,
        })
        .filter_map(|args| {
            let mut args = args.split(",");
            match (args.next(), args.next()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            }
        })
        .filter_map(|(a, b)| match (a.parse::<i32>(), b.parse::<i32>()) {
            (Ok(a), Ok(b)) => Some((a, b)),
            _ => None,
        })
        .map(|(a, b)| a * b)
        .sum()
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
