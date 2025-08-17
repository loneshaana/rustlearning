/*
    This will be max-heap
*/

use std::mem::swap;

pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        self.sift_up(0, old_len);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut last_item| {
            if !self.is_empty() {
                swap(&mut last_item, &mut self.data[0]);
                self.sift_down_to_bottom(0);
            }
            last_item
        })
    }

    fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    fn sift_up(&mut self, start: usize, mut pos: usize) {
        while pos > start {
            let parent = (pos - 1) / 2;
            unsafe {
                if self.data.get_unchecked(pos) < self.data.get_unchecked(parent) {
                    break;
                }
                self.data.swap(pos, parent);
                pos = parent;
            }
        }
    }

    fn sift_down_to_bottom(&mut self, pos: usize) {
        let end = self.len();

        let mut child = 2 * pos + 1;

        while child <= end.saturating_sub(2) {
            let left = unsafe { self.data.get_unchecked(child) };
            let right = unsafe { self.data.get_unchecked(child + 1) };
            child += (left <= right) as usize;

            self.data.swap(pos, child);
            child = 2 * child + 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let heap: BinaryHeap<i32> = BinaryHeap::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_new_with_capacity() {
        let heap: BinaryHeap<i32> = BinaryHeap::new_with_capacity(10);
        assert!(heap.is_empty());
        assert_eq!(heap.data.capacity(), 10);
    }

    #[test]
    fn test_is_empty() {
        let heap: BinaryHeap<i32> = BinaryHeap::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_pop_empty() {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_pop_non_empty() {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.peek(), Some(&3));
    }
}
