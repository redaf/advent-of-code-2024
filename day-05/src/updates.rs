pub type Update = Vec<u8>;
pub struct Updates(Vec<Update>);

// fn iter<'a>(s: &'a str) -> impl Iterator<Item = Update> + 'a {
//     s.lines().filter(|line| line.contains(',')).map(|line| {
//         line.split(',')
//             .flat_map(|item| item.parse::<u8>())
//             .collect()
//     })
// }

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
