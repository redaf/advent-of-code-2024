use rules::PageOrderingRules;
use updates::{Update, Updates};

mod rules;
mod updates;

fn main() {
    let input = include_str!("../input.txt");
    let rules: PageOrderingRules = input.into();
    let updates: Updates = input.into();

    println!(
        "Part 1 - Sum of middle pages: {}",
        middle_pages_sum(updates, &rules)
    );
}

fn update_is_correct(update: &Update, rules: &PageOrderingRules) -> bool {
    let update = update.as_slice();

    for (i, page) in update.iter().enumerate() {
        let l_vals = &update[..i];
        let r_vals = &update[i + 1..];
        let l_rules = rules.before(*page);
        let r_rules = rules.after(*page);

        // println!("{:?} -- {} -- {:?}", l_vals, page, r_vals);
        // println!("{}|{:?} -- {:?}|{}", page, r_rules, l_rules, page);

        let l_ok = l_vals.iter().all(|p| !r_rules.contains(p));
        let r_ok = r_vals.iter().all(|p| !l_rules.contains(p));

        if !l_ok || !r_ok {
            return false;
        }
    }

    true
}

fn middle_pages_sum(updates: Updates, rules: &PageOrderingRules) -> usize {
    updates
        .into_iter()
        .filter(|update| update_is_correct(update, &rules))
        .map(|update| {
            debug_assert!(update.len() % 2 == 1);
            update[update.len() / 2] as usize
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rules::PageOrderingRules;
    use updates::Updates;

    const EXAMPLE_1: &str = include_str!("../example_1.txt");

    #[test]
    fn rules_parser_test() {
        let rules: PageOrderingRules = EXAMPLE_1.into();
        assert_eq!(rules.get(0), Some(&(47, 53)));
        assert_eq!(rules.get(1), Some(&(97, 13)));
        assert_eq!(rules.get(6), Some(&(75, 53)));
        assert_eq!(rules.get(20), Some(&(53, 13)));
    }

    #[test]
    fn updates_parser_test() {
        let updates: Updates = EXAMPLE_1.into();
        assert_eq!(updates.get(0), Some(&vec![75, 47, 61, 53, 29]));
        assert_eq!(updates.get(2), Some(&vec![75, 29, 13]));
        assert_eq!(updates.get(5), Some(&vec![97, 13, 75, 29, 47]));
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
    fn example_1_update_is_correct() {
        let rules: PageOrderingRules = EXAMPLE_1.into();
        let updates: Updates = EXAMPLE_1.into();
        assert_eq!(true, update_is_correct(updates.get(0).unwrap(), &rules));
        assert_eq!(true, update_is_correct(updates.get(1).unwrap(), &rules));
        assert_eq!(true, update_is_correct(updates.get(2).unwrap(), &rules));
        assert_eq!(false, update_is_correct(updates.get(3).unwrap(), &rules));
        assert_eq!(false, update_is_correct(updates.get(4).unwrap(), &rules));
        assert_eq!(false, update_is_correct(updates.get(5).unwrap(), &rules));
    }

    #[test]
    fn example_1_sum() {
        let rules: PageOrderingRules = EXAMPLE_1.into();
        let updates: Updates = EXAMPLE_1.into();
        let sum: usize = middle_pages_sum(updates, &rules);
        assert_eq!(143, sum);
    }

    #[test]
    fn testtt() {
        assert_eq!(true, [11, 2, 3].iter().all(|&n| n > 0));
        let s = [1, 2, 3];
        let x = &s[..0];
        assert_eq!(true, x.iter().all(|&n| n > 0));
    }
}
