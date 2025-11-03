// 1578. 使绳子变成彩色的最短时间
pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let mut ch = colors.chars().collect::<Vec<char>>();
    let mut left = 0usize;
    let l = colors.len();
    let mut ans = 0;
    while left < l {
        let mut k = left + 1;
        let mut max = needed_time[left];
        let mut sum = needed_time[left];
        while k < l && ch[k] == ch[k - 1] {
            max = std::cmp::max(max, needed_time[k]);
            sum += needed_time[k];
            k += 1;
        }
        if k - 1 > left {
            // 存在相同的颜色
            // 保留一个最大值
            ans += sum - max
        }
        // left从k开始
        left = k;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_min_cost() {
        assert!(min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 5]) == 5);
    }
}
