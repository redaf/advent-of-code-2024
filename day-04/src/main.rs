use core::str;

fn main() {
    let word_search = include_str!("../input.txt");
    println!("Part 1 - XMAS count: {}", xmas_count_1(word_search));
    println!("Part 2 - XMAS count: {}", xmas_count_2(word_search));
}

struct WordSearch<'a> {
    bytes: &'a [u8],
    side: usize,
}

impl<'a> WordSearch<'a> {
    fn new(s: &'a str) -> Self {
        let side = s.lines().count();
        assert!(s.lines().all(|line| line.len() == side));
        WordSearch {
            bytes: s.as_bytes(),
            side,
        }
    }

    fn side(&self) -> usize {
        self.side
    }

    fn char_at(&self, x: i32, y: i32) -> Option<u8> {
        let s = self.side as i32;
        if x >= 0 && x < s && y >= 0 && y < s {
            let k = x + (s + 1) * y;
            Some(self.bytes[k as usize])
        } else {
            None
        }
    }

    fn count_word_occurences(&self, word: &[u8]) -> usize {
        const DIRS: [(i32, i32); 8] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        let mut count = 0;
        for y in 0..(self.side) as i32 {
            for x in 0..(self.side) as i32 {
                let center = self.char_at(x, y).unwrap_or(0);
                if center != word[0] {
                    continue;
                }
                let mut dir_alive = [true; DIRS.len()];
                for word_index in 1..word.len() as i32 {
                    for d in 0..DIRS.len() {
                        if !dir_alive[d] {
                            continue; // this direction is out
                        };
                        let dx = x + DIRS[d].0 * word_index;
                        let dy = y + DIRS[d].1 * word_index;
                        let char_in_dir = self.char_at(dx, dy).unwrap_or(0);
                        if char_in_dir != word[word_index as usize] {
                            dir_alive[d] = false;
                        }
                    }
                }
                count += dir_alive.iter().filter(|&&alive| alive).count();
            }
        }
        count
    }

    fn count_x_occurences(&self) -> usize {
        let mut count = 0;
        for y in 0..(self.side) as i32 {
            for x in 0..(self.side) as i32 {
                let center = self.char_at(x, y).unwrap_or(0) as char;
                if center != 'A' {
                    continue;
                }
                let north_east = self.char_at(x + 1, y + 1).unwrap_or(0) as char;
                let south_west = self.char_at(x - 1, y - 1).unwrap_or(0) as char;
                let nesw = match (north_east, south_west) {
                    ('M', 'S') | ('S', 'M') => true,
                    _ => false,
                };
                let north_west = self.char_at(x - 1, y + 1).unwrap_or(0) as char;
                let south_east = self.char_at(x + 1, y - 1).unwrap_or(0) as char;
                let nwse = match (south_east, north_west) {
                    ('M', 'S') | ('S', 'M') => true,
                    _ => false,
                };
                count += if nesw && nwse { 1 } else { 0 }
            }
        }
        count
    }
}

fn xmas_count_1(word_search: &str) -> usize {
    let ws = WordSearch::new(word_search);
    ws.count_word_occurences("XMAS".as_bytes())
}

fn xmas_count_2(word_search: &str) -> usize {
    let ws = WordSearch::new(word_search);
    ws.count_x_occurences()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = //
        "MMMSXXMASM\n\
MSAMXMSMSA\n\
AMXSXMAAMM\n\
MSAMASMSMX\n\
XMASAMXAMM\n\
XXAMMXXAMA\n\
SMSMSASXSS\n\
SAXAMASAAA\n\
MAMMMXMMMM\n\
MXMXAXMASX";

    #[test]
    fn tiny_search() {
        let tiny: &str = ".X....\n\
.SAMX.\n\
.A..A.\n\
XMAS.S\n\
.X....\n\
......";
        let ws = WordSearch::new(tiny);
        assert_eq!(6, ws.side());
        assert_eq!(3, ws.count_word_occurences("XMAS".as_bytes()));
    }

    #[test]
    fn word_search() {
        let ws = WordSearch::new(EXAMPLE_1);
        assert_eq!(10, ws.side());
        assert_eq!(18, ws.count_word_occurences("XMAS".as_bytes()));
    }

    #[test]
    fn x_mas_search() {
        let word_search = include_str!("../example_2.txt");
        let ws = WordSearch::new(word_search);
        assert_eq!(10, ws.side());
        assert_eq!(9, ws.count_x_occurences());
    }
}
