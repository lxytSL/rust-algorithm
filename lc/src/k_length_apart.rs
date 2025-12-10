// 1437. 是否所有 1 都至少相隔 k 个元素
pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut left: i32 = -1;
    for (i, num) in nums.iter().enumerate() {
        let i = i as i32;
        if *num == 1 && left >= 0 {
            // 不是第一个1
            if i - left <= k {
                return false;
            } else {
                left = i;
            }
        } else if *num == 1 && left < 0 {
            // 是第一个1
            left = i;
        }
    }
    return true;
}

mod test {
    use super::*;
    #[test]
    fn test_k_length_apart() {
        assert!(k_length_apart(vec![1, 0, 0, 1, 0, 1], 2) == false);
    }
}
