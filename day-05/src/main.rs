fn main() {
    println!("Hello, world!");
}

struct PageOrderingRules(Vec<(u8, u8)>);

impl From<&str> for PageOrderingRules {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example_1.txt");

    fn parse_rules(safety_manual: &str) -> Vec<(u8, u8)> {
        safety_manual
            .lines()
            .filter(|line| line.contains('|'))
            .filter_map(|line| {
                let mut split = line.split('|');
                match (split.next(), split.next()) {
                    (Some(a), Some(b)) => Some((a, b)),
                    _ => None,
                }
            })
            .filter_map(|(a, b)| match (a.parse::<u8>(), b.parse::<u8>()) {
                (Ok(a), Ok(b)) => Some((a, b)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn rules_parser_test() {
        let rules = parse_rules(EXAMPLE_1);
        assert_eq!(rules[0], (47, 53));
    }

    fn update_order_is_correct(rules: &[(u8, u8)], update: &[u8]) -> bool {
        false
    }

    // In the above example, the first update (75,47,61,53,29) is in the right order:
    //
    //     - 75 is correctly first because there are rules that put each other page
    //       after it: 75|47, 75|61, 75|53, and 75|29.
    //     - 47 is correctly second because 75 must be before it (75|47) and every
    //       other page must be after it according to 47|61, 47|53, and 47|29.
    //     - 61 is correctly in the middle because 75 and 47 are before it (75|61
    //       and 47|61) and 53 and 29 are after it (61|53 and 61|29).
    //     - 53 is correctly fourth because it is before page number 29 (53|29).
    //     - 29 is the only page left and so is correctly last.
    //
    // Because the first update does not include some page numbers, the ordering rules involving those missing page numbers are ignored.
    //
    #[test]
    fn ex_1_first_update() {
        let rules = [(75, 47), (75, 61), (75, 53), (75, 29)];
        let update = [75, 47, 61, 53, 29];
        assert_eq!(true, update_order_is_correct(&rules, &update));
    }
}
