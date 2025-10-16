// 3349. 检测相邻递增子数组 I

pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    let mut cnt = 1;
    let mut precnt = 0;
    let mut ans = 0;

    for i in 1..n {
        if nums[i] > nums[i - 1] {
            cnt += 1;
        } else {
            precnt = cnt;
            cnt = 1;
        }
        ans = ans.max(precnt.min(cnt));
        ans = ans.max(cnt / 2);
    }

    ans >= k
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_has_increasing_subarrays() {
        assert!(has_increasing_subarrays(vec![19, 5], 1) == true);
        assert!(has_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3) == true);
        assert!(has_increasing_subarrays(vec![-15, 3, 16, 0], 2) == false);
    }
}
