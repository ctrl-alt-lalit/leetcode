use std::{cell::RefCell, cmp::Reverse, collections::BinaryHeap, rc::Rc};

// Leetcode TreeNode implementation
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MinHeap<T: Ord>(BinaryHeap<Reverse<T>>);

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(BinaryHeap::with_capacity(capacity))
    }

    pub fn peek(&self) -> Option<&T> {
        match self.0.peek() {
            Some(Reverse(ref inner)) => Some(inner),
            None => None,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.0.pop() {
            Some(Reverse(inner)) => Some(inner),
            None => None,
        }
    }
}

impl<T: Ord> From<Vec<T>> for MinHeap<T> {
    fn from(mut value: Vec<T>) -> Self {
        // SAFETY: Reverse<T> has the same byte representation as T, so it can be safely transmuted.
        // SAFETY: All other Vec memory layout invariants are upheld since we're transmuting from another Vec.
        let reversed = unsafe {
            Vec::from_raw_parts(
                value.as_mut_ptr() as *mut Reverse<T>,
                value.len(),
                value.capacity(),
            )
        };
        Self(BinaryHeap::from(reversed))
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct BitSet(Vec<usize>);

impl BitSet {
    const WS: usize = std::mem::size_of::<usize>();

    pub fn with_len(length: usize) -> Self {
        Self(vec![0; length / Self::WS + 1])
    }

    pub fn get(&self, i: usize) -> bool {
        let (word_idx, bitmask) = Self::parse_index(i);
        ((self.0[word_idx] & bitmask) > 0)
    }

    pub fn set(&mut self, i: usize, b: bool) {
        let (word_idx, bitmask) = Self::parse_index(i);
        if b {
            self.0[word_idx] |= bitmask
        } else {
            self.0[word_idx] &= !bitmask
        }
    }

    fn parse_index(i: usize) -> (usize, usize) {
        (i / Self::WS, 1usize << (i % Self::WS))
    }

    pub fn iter_nums(&self) -> BitSetNumIter {
        BitSetNumIter {
            data: self,
            index: -1,
        }
    }

    pub fn count(&self) -> u32 {
        self.0.iter().fold(0, |acc, w| acc + w.count_ones())
    }

    /// Returns next `Some(j)` where `Bitset.get(j)` is `true`, or `None`. `j` may equal `i`.
    fn next_true_index(&self, i: usize) -> Option<usize> {
        if i >= self.0.len() * Self::WS {
            return None;
        }

        let word_idx = i / Self::WS;
        let bit_idx = i % Self::WS;
        let curr_mask = !((1 << bit_idx) - 1);

        // check current word first
        if (self.0[word_idx] & curr_mask) > 0 {
            let lsb_mask = curr_mask & (!curr_mask + 1);
            return Some(word_idx * Self::WS + lsb_mask.trailing_zeros() as usize);
        }

        // check subsequent words
        for wi in word_idx + 1..self.0.len() {
            let word = self.0[wi];
            if word > 0 {
                let lsb_mask = word & (!word + 1);
                return Some(wi * Self::WS + lsb_mask.trailing_ones() as usize);
            }
        }

        return None;
    }
}

struct BitSetNumIter<'a> {
    data: &'a BitSet,
    index: i32,
}

impl<'a> Iterator for BitSetNumIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_i = self.data.next_true_index((self.index + 1) as usize)?;
        self.index = next_i as i32;
        Some(next_i)
    }
}
