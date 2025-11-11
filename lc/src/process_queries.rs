use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::vec;

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(c: usize) -> Self {
        let mut parent = vec![0; c + 1];
        for i in 1..=c {
            parent[i] = i;
        }
        Self { parent }
    }

    // 查找
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    // 合并
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
        }
    }
}

// 3607. 电网维护
pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let c = c as usize;
    // 创建并查集
    let mut dsu = DSU::new(c);
    for conn in connections.iter() {
        dsu.union(conn[0] as usize, conn[1] as usize);
    }
    // 存活
    let mut life = vec![true; c + 1];
    let mut mp_que = HashMap::new();
    let mut root_set = HashSet::new();
    // 每个并查集维护最小堆
    for i in 1..=c {
        // 找到父节点
        let root = dsu.find(i);
        mp_que
            .entry(root)
            .or_insert(BinaryHeap::new())
            .push(Reverse(i));
        root_set.insert(root);
    }
    // 将root也要插入
    for &root in root_set.iter() {
        mp_que
            .entry(root)
            .or_insert(BinaryHeap::new())
            .push(Reverse(root));
    }
    let mut ans: Vec<i32> = vec![];
    for item in queries.iter() {
        let op = item[0];
        let node = item[1];
        if op == 1 {
            // 存活，自己维护
            if life[node as usize] {
                ans.push(node);
                continue;
            }
            // 找到并查集
            let root = dsu.find(node as usize);
            // 找到最小堆
            let min_heap = mp_que.get_mut(&root).unwrap();
            while let Some(&Reverse(min)) = min_heap.peek() {
                if life[min] {
                    ans.push(min as i32);
                    break;
                } else {
                    min_heap.pop();
                }
            }
            if min_heap.is_empty() {
                ans.push(-1);
            }
        } else {
            // op == 2
            life[node as usize] = false;
        }
    }
    ans
}

mod test {
    use super::*;
    #[test]
    fn test_process_queries() {
        assert!(
            process_queries(
                3,
                vec![vec![1, 2], vec![2, 3]],
                vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![2, 1], vec![1, 1],]
            ) == vec![1, 2, 3, 2]
        );
    }
}
