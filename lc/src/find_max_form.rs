// 474. 一和零
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1 as usize]; m + 1];

    for s in strs.iter() {
        let (zeros, ones) = get_zeros_ones(s);
        let zeros = zeros as usize;
        let ones = ones as usize;
        for mm in (zeros..=m).rev() {
            for nn in (ones..=n).rev() {
                dp[mm][nn] = dp[mm][nn].max(dp[mm - zeros][nn - ones] + 1);
            }
            dbg!(&dp);
        }
    }
    dp[m][n]
}

fn get_zeros_ones(s: &str) -> (i32, i32) {
    let mut zeros = 0;
    let mut ones = 0;
    for b in s.chars() {
        if b == '0' {
            zeros += 1;
        } else {
            ones += 1;
        }
    }
    (zeros, ones)
}

mod test {
    use super::*;
    #[test]
    fn test_find_max_form() {
        assert!(
            find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            ) == 4
        );
    }
}
