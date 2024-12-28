pub type Rule = (u8, u8);
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

    pub fn before(&self, page_number: u8) -> Vec<u8> {
        self.0
            .iter()
            .filter_map(|(l, r)| {
                if *r == page_number {
                    return Some(*l);
                } else {
                    return None;
                }
            })
            .collect()
    }

    pub fn after(&self, page_number: u8) -> Vec<u8> {
        self.0
            .iter()
            .filter_map(|(l, r)| {
                if *l == page_number {
                    return Some(*r);
                } else {
                    return None;
                }
            })
            .collect()
    }
}

impl IntoIterator for PageOrderingRules {
    type Item = Rule;
    type IntoIter = std::vec::IntoIter<Rule>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.0.into_iter()
    }
}
