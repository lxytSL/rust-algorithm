// 3577. 统计计算机解锁顺序排列数

pub fn count_permutations(complexity: Vec<i32>) -> i32 {
    // 要么0要么Ann
    let tag = complexity[0];
    let l = complexity.len();
    let mode = 1000000007;
    let mut ans: i64 = 0;
    for i in 1..=l - 1 {
        // 严格小于第0个数，小于等于前一个数
        if complexity[i] > tag {
            ans = (ans * i as i64) % mode;
        } else {
            return 0;
        }
    }
    ans as i32
}

mod test {
    use super::*;
    #[test]
    fn test_count_permutations() {
        assert!(count_permutations(vec![2,68,61]) == 2);
    }
}
