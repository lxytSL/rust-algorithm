

// 2221. 数组的三角和
pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
    let l = nums.len();
    for i in 0..l - 1 {
        for j in 0..l - i - 1{
            nums[j] = (nums[j] + nums[j + 1])  % 10
        }
    }
    nums[0]
}
