fn main() {
    let input = include_str!("../input.txt");

    let sum_part_1 = part_1_sum_of_uncorrupted_muls(input);
    println!("Part 1 - Sum of uncorrupted multiplications: {sum_part_1}");
}

fn part_1_sum_of_uncorrupted_muls(memory: &str) -> i32 {
    memory
        .split("mul(")
        .filter_map(|args| {
            let args = match args.find(',') {
                Some(i) => args.split_at(i),
                None => ("", ""),
            };
            let a = args.0;
            if a.is_empty() || !a.chars().all(char::is_numeric) {
                return None;
            }
            let b = &args.1[1..];
            let b = match b.find(')') {
                Some(i) => &b[..i],
                None => "",
            };
            if b.is_empty() || !b.chars().all(char::is_numeric) {
                return None;
            }

            Some((a, b))
        })
        .filter(|(a, b)| a.len() <= 3 && b.len() <= 3)
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
