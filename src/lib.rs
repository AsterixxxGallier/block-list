#![feature(lint_reasons)]

use std::ops::{Index, IndexMut};
use crate::block::{Height, HeightAtZero, Storage};

/// A `BlockList` consists of blocks.
///
/// Every block has a height, a start index, an end index, a size, and a value.
///
/// The size of a block is 2 to the power of its height.
///
/// The end index is the start index plus the size.
///
/// The number of trailing zeros of the start index of a block must be greater than its height.
///
/// The value of a block is the combination of the individual values at all indices from its start
/// to its end (exclusive). In this implementation, it is their sum.
///
/// Only the values of the blocks are stored in memory, in a list. This implementation uses a Vec.
/// The values are stored at the end index of the block minus 1.
///
/// If multiple blocks would have the same end index, only the block with the greatest height exists
/// and only its value is stored.
///
/// ```text
/// height 3: 000---------------------------------------------000
/// height 2: 000---------------------100                     000---------------------100
/// height 1: 000---------010         100---------110         000---------010         100---------110
/// height 0: 000---001   010---011   100---101   110---111   000---001   010---011   100---101   110---111
/// ```
// BlockList is of elements where + and - are defined such that A + B - B = A
// PartialBlockList is of elements where only + is defined
// TODO: think about BlockList variant where negative values are allowed
pub struct BlockList {
    values: Vec<usize>,
    highest_block: HeightAtZero
}

impl Default for BlockList {
    fn default() -> Self {
        Self {
            values: vec![],
            highest_block: HeightAtZero(Height(0))
        }
    }
}

impl BlockList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: usize) {
        let storage = Storage(self.values.len());
        let sum_of_children: usize =
            storage.children().map(|storage| self.values[storage.get()]).sum();
        self.values.push(sum_of_children + value);
        if self.values.len().is_power_of_two() {
            self.highest_block = self.highest_block.parent();
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // FIXME panics when not all small siblings are in bounds
    pub fn sum(&self) -> usize {
        self[self.highest_block.storage()] +
            self.highest_block
                .small_siblings()
                .map(|block| self[block.storage()])
                .sum::<usize>()
    }
}

impl Index<Storage> for BlockList {
    type Output = usize;

    fn index(&self, index: Storage) -> &Self::Output {
        &self.values[index.get()]
    }
}

impl IndexMut<Storage> for BlockList {
    fn index_mut(&mut self, index: Storage) -> &mut Self::Output {
        &mut self.values[index.get()]
    }
}

mod block;

#[cfg(test)]
mod tests {
    use crate::BlockList;

    #[test]
    fn it_works() {
        let mut list = BlockList::new();
        for _ in 0..1000 {
            list.push(1);
        }
        println!("{:?}", list.values);
    }
}
