pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
    // 倒序
    nums.sort_by(|a, b| b.cmp(a));
    // 找到第一个符合条件的三角形
    for i in 2..nums.len() {
        // 因为是倒叙，a > b > c，
        // 直接判断相邻的b+c是否>a，即可，
        // 如果相邻的b+c都小于a, 其他的肯定是小于的
        if nums[i] + nums[i-1] > nums[i-2] {
            return nums[i] + nums[i-1] + nums[i-2]
        }
    }
    0
}
