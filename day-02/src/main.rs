fn main() {
    println!("Hello, world!");
}

fn safe_cond_1(report: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;
    // So, a report only counts as safe if both of the following are true:
    //
    //  - The levels are either all increasing or all decreasing.
    //  - Any two adjacent levels differ by at least one and at most three.

    const EXAMPLE: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn example_reports_count() {
        assert_eq!(EXAMPLE.lines().count(), 6);
    }

    //  - The levels are either all increasing or all decreasing.
    #[test]
    fn safe_condition_1() {
        let mut lines = EXAMPLE.lines();
        assert_eq!(safe_cond_1(lines.next().unwrap()), true);
        assert_eq!(safe_cond_1(lines.next().unwrap()), false);
        assert_eq!(safe_cond_1(lines.next().unwrap()), false);
        assert_eq!(safe_cond_1(lines.next().unwrap()), false);
        assert_eq!(safe_cond_1(lines.next().unwrap()), false);
        assert_eq!(safe_cond_1(lines.next().unwrap()), true);
    }
}
