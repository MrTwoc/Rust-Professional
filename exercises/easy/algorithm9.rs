/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// 将元素添加到堆中
    pub fn add(&mut self, value: T) {
        // 将元素添加到堆的末尾
        self.items.push(value);
        // 增加堆的大小
        self.count += 1;
        // 从最后一个元素开始向上调整堆
        self.switch(self.count);
    }

    /// 从指定索引开始向上调整堆
    fn switch(&mut self, idx: usize) {
        let mut idx = idx;
        // 当当前节点不是根节点且当前节点的值小于其父节点的值时
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
            // 获取父节点的索引
            let parent_index = self.parent_idx(idx);
            // 交换当前节点和其父节点的位置
            self.items.swap(idx, parent_index);
            // 更新当前节点的索引为其父节点的索引
            idx = self.parent_idx(idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
    /// 获取当前节点的最小子节点的索引
    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 获取当前节点的左子节点索引
        let left = self.left_child_idx(idx);
        // 获取当前节点的右子节点索引
        let right = self.right_child_idx(idx);

        // 如果右子节点存在且右子节点的值小于左子节点的值，则返回右子节点的索引
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        // 否则返回左子节点的索引
        } else {
            left
        }
    }

    /// 从指定索引开始向下调整堆
    fn re_switch(&mut self, idx: usize) {
        let mut idx = idx;
        // 当当前节点存在子节点时
        while self.children_present(idx) {
            // 获取当前节点的最小子节点的索引
            let smallest_child = self.smallest_child_idx(idx);
            // 如果最小子节点的值小于当前节点的值，则交换它们的位置
            if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                self.items.swap(idx, smallest_child);
                idx = smallest_child;
            // 否则跳出循环
            } else {
                break;
            }
        }
    }

    /// 移除并返回堆顶元素
    pub fn pop(&mut self) -> Option<T> {
        // 如果堆为空，则返回None
        if self.is_empty() {
            return None;
        }
        // 将堆顶元素与最后一个元素交换位置
        self.items.swap(1, self.count);
        // 减少堆的大小
        self.count -= 1;
        // 从堆顶开始向下调整堆
        self.re_switch(1);
        // 移除并返回最后一个元素
        self.items.pop()
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}