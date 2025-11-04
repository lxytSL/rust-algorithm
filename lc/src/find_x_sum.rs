use std::collections::HashMap;

// 3318. 计算子数组的 x-sum I
pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let l = nums.len();
    let k = k as usize;
    let x = x as usize;
    let mut answer = vec![0i32; l - k + 1];
    let mut pair = HashMap::new();
    // 统计answer[0]
    let mut left = 0;
    let mut right = 0;
    while right < l {
        let win = right - left + 1;
        *pair.entry(&nums[right]).or_insert(0) += 1;
        if win == k {
            // 计算answer[left]的值
            let mut items: Vec<(&i32, i32)> = pair.iter().map(|(k, v)| (*k, *v)).collect();
            items.sort_unstable_by(|a, b| {
                b.1.cmp(&a.1) // value 降序
                    .then_with(|| b.0.cmp(a.0)) // key 降序
            });
            answer[left] = items.into_iter().take(x).map(|(k, v)| v * k).sum();
            *pair.entry(&nums[left]).or_insert(0) -= 1;
            left += 1;
        }
        right += 1;
    }
    answer
}

mod test {
    use super::*;
    #[test]
    fn test_find_x_sum() {
        assert!(find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2) == vec![6, 10, 12]);
    }
}
