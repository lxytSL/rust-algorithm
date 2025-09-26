// 611. 有效三角形的个数

pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let l = nums.len();
    let mut ans = 0;
    for i in 0..l - 2 {
        for j in i + 1..l - 1 {
            let target = nums[i] + nums[j];
            let mut left = j;
            let mut right = l - 1;
            // 第一个target小于的数的索引
            let right = get_last_min(&nums, j + 1, l - 1, target);
            ans += right - j;
        }
    }
    ans as i32
}

fn get_last_min(nums: &Vec<i32>, mut left: usize, mut right: usize, target: i32) -> usize {
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] >= target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_last_min_test() {
        assert!(get_last_min(&vec![2, 2, 3, 4], 2, 3, 4) == 2);
    }
    #[test]
    fn triangle_number_test() {
        assert!(triangle_number(vec![2, 2, 3, 4]) == 3);
    }
}
