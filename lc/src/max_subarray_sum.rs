// 3381. 长度可被 K 整除的子数组的最大元素和

pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let l = nums.len();
    let k = k as usize;
    // 表示长度为i的前缀和。从1开始
    let mut pre_sum = 0i64;
    let mut max = std::i64::MIN;
    // k个桶， 存放mod/k的最小值
    let mut min_i = vec![std::i64::MAX/2; k];
    for (i, v) in nums.iter().enumerate() {
        pre_sum += *v as i64;
        // i-bucket = n * k;
        let bucket = i % k;
        // pre_sum - min_i[bucket] 这就是区间为n * k的和
        max = max.max(pre_sum - min_i[bucket]);
        min_i[bucket] = min_i[bucket].min(pre_sum);
    }

    max
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_subarray_sum() {
        assert!(max_subarray_sum(vec![1, 2, 3, 4, 5], 3) == 12);
    }
}
