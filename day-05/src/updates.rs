pub type Update = Vec<u8>;
#[derive(Clone)]
pub struct Updates(Vec<Update>);

impl From<&str> for Updates {
    fn from(safety_manual: &str) -> Self {
        Updates(
            safety_manual
                .lines()
                .filter(|line| line.contains(','))
                .map(|line| {
                    line.split(',')
                        .flat_map(|item| item.parse::<u8>())
                        .collect()
                })
                .collect(),
        )
    }
}

impl Updates {
    #[cfg(test)]
    pub fn get(&self, index: usize) -> Option<&Update> {
        self.0.get(index)
    }
}

impl IntoIterator for Updates {
    type Item = Update;
    type IntoIter = std::vec::IntoIter<Update>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.0.into_iter()
    }
}
