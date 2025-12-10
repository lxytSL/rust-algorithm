// 2154. 将找到的值乘以 2
pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let mut nums = nums;
    let mut original = original;
    nums.sort_unstable();
    while nums.binary_search(&original).is_ok() {
        original *= 2;
    }
    original
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_find_final_value() {
        assert!(find_final_value(vec![1, 3, 4, 2, 5], 2) == 8);
    }
}
