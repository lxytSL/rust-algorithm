use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct NumberContainers {
    // 数字对应的最小堆
    heaps: HashMap<i32, BinaryHeap<Reverse<i32>>>,
    index_num: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            heaps: HashMap::new(),
            index_num: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        // 插入数据
        self.index_num.insert(index, number);
        // 插入num的index, 保持index升序
        self.heaps
            .entry(number)
            .or_insert(BinaryHeap::new())
            .push(Reverse(index));
    }

    fn find(&mut self, number: i32) -> i32 {
        if let Some(heap) = self.heaps.get_mut(&number) {
            while let Some(&Reverse(min_index)) = heap.peek() {
                if self.index_num.get(&min_index) != Some(&number) {
                    heap.pop();
                } else {
                    return min_index;
                }
            }
        }
        -1
    }
}
