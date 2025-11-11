// 3542. 将所有元素变为 0 的最少操作次数
pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let mut ans = 0;
    for i in nums {
        while stack.last().map_or(false, |&x| x < i) {
            stack.pop();
        }
        if i == 0 {
            continue;
        }
        while stack.last().map_or(false, |&x| x > i) {
            stack.push(i);
            ans += 1;
        }
    }
    ans
}
