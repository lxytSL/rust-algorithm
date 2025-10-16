// 2598. 执行操作后的最大 MEX
pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let mut bucket = vec![0; value as usize];
    nums.iter().for_each(|&num| {
        // 去正模
        bucket[(((num % value) + value) % value) as usize] += 1;
    });
    let mut k = 0;
    while bucket[(k % value) as usize] > 0 {
        bucket[(k % value) as usize] -= 1;
        k += 1;
    }
    k
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_find_smallest_integer() {
        // assert!(find_smallest_integer(vec![1,-10,7,13,6,8], 5) == 4);
        assert!(find_smallest_integer(vec![3, 0, 3, 2, 4, 2, 1, 1, 0, 4], 5) == 10);
    }
}
