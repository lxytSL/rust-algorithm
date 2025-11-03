pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    // 分组亦或
    let l = nums.len() - 2;
    let mut flag = 0;
    for i in nums.iter() {
        flag = flag ^ i;
    }
    for i in 0..l as i32 {
        flag ^= i;
    }

    let mut k = 1;
    // 判断第几位为1;
    while flag & k == 0 {
        k = k << 1;
    }

    // 分组
    let mut a = 0;
    let mut b = 0;
    for i in nums {
        if i & k == 0 {
            // 第k位没1
            a ^= i;
        } else {
            b ^= i;
        }
    }
    for i in 0..l as i32 {
        if i & k == 0 {
            a ^= i;
        } else {
            b ^= i;
        }
    }
    vec![a, b]
}

mod test {
    use super::*;

    #[test]
    fn test_get_sneaky_numbers() {
        let nums = vec![0, 1, 2, 1, 2, 3, 4];
        let result = get_sneaky_numbers(nums);
        assert_eq!(result, vec![1, 2]);
    }
}
