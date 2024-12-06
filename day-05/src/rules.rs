type Rule = (u8, u8);
pub struct PageOrderingRules(Vec<Rule>);

impl From<&str> for PageOrderingRules {
    fn from(safety_manual: &str) -> Self {
        Self(
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
                .collect(),
        )
    }
}

impl PageOrderingRules {
    #[cfg(test)]
    pub fn get(&self, index: usize) -> Option<&Rule> {
        self.0.get(index)
    }
}
