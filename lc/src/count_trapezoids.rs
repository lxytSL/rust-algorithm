// 3623. 统计梯形的数目 I

use std::collections::HashMap;

pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    // 按y相同进行分组
    let mut group_y = HashMap::new();
    for item in points.iter() {
        *group_y.entry(item[1]).or_insert(0) += 1i64
    }
    let mut ans: i64 = 0;
    let mode: i64 = 1000000007;
    let mut sum = 0i64;
    for num in group_y.values() {
        // 相同y的边的条数
        let edge = num / 2 * (num - 1);
        ans = (ans + edge * sum) % mode;
        sum += edge;
    }
    ans as i32
}
