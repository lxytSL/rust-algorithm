// 3432. 统计元素和差值为偶数的分区方案

pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let l = nums.len();
    let mut right = nums.iter().sum::<i32>();
    let mut ans = 0;
    let mut left = 0;
    for v in nums[0..l-1].iter() {
        right -= v;
        left += v;
        if (right - left).abs() % 2 == 0 {
            ans += 1;
        }
    }
    ans
}

mod test {
    use super::*;
    #[test]
    fn test_count_partitions() {
        assert!(count_partitions(vec![1, 2, 2]) == 0);
        assert!(count_partitions(vec![10, 10, 3, 7, 6]) == 4);
    }
}
