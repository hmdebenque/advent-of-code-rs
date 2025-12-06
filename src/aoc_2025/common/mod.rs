// Regroup code used in several puzzles

use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl<'a> IntoIterator for &'a Range {

    type Item = &'a usize;
    type IntoIter = std::array::IntoIter<&'a usize, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.start, &self.end].into_iter()
    }
}
