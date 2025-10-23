// 370， 区间加法
pub fn range_addition(n: i32, intervals: Vec<(i32, i32, i32)>) -> Vec<i32> {
    // 建立差分数组
    let mut diff = vec![0; n as usize];
    for interval in intervals {
        // 建立区间开始的影响
        diff[interval.0 as usize] += interval.2;
        if interval.1 + 1< n {
            // 区间结束要减去影响
            diff[(interval.1 + 1) as usize] -= interval.2;
        }
    }
    // 遍历差分数组
    // 计算前缀和，还原结果
    for i in 1..n {
        diff[i as usize] += diff[(i - 1) as usize];
    }
    diff
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_range_addition() {
        assert!(range_addition(5, vec![(1, 3, 2), (2, 4, 3), (0, 2, -2)]) == vec![-2, 0, 3, 5, 3]);
    }
}
