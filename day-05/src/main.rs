fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example_1.txt");

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
