fn main() {
    let input = include_str!("../input.txt");

    let count = input
        .lines()
        .filter(|report: &&str| report_is_safe(*report))
        .count();
    println!("Safe reports count: {count}");
}

fn report_is_safe(report: &str) -> bool {
    let numbers = report
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());
    // all increasing
    let inc = numbers.clone().is_sorted_by(|a, b| {
        let diff = b - a;
        (diff >= 1) && (diff <= 3)
    });
    // all decreasing
    let dec = numbers.is_sorted_by(|a, b| {
        let diff = a - b;
        (diff >= 1) && (diff <= 3)
    });
    inc || dec
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
    fn report_is_safe_test() {
        let mut lines = EXAMPLE.lines();
        assert_eq!(report_is_safe(lines.next().unwrap()), true);
        assert_eq!(report_is_safe(lines.next().unwrap()), false);
        assert_eq!(report_is_safe(lines.next().unwrap()), false);
        assert_eq!(report_is_safe(lines.next().unwrap()), false);
        assert_eq!(report_is_safe(lines.next().unwrap()), false);
        assert_eq!(report_is_safe(lines.next().unwrap()), true);
    }
}