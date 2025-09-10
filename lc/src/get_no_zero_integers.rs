use std::vec;

pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
    let mut i = 1;
    while i < n {
        let j = n - i;
        // 判断j是否不存在0
        if !i.to_string().contains('0') && !j.to_string().contains('0') {
            return vec![i, j];
        }
        i += 1;
    }
    vec![]
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_no_zero_integers_test() {
        assert!(get_no_zero_integers(2) == vec![1, 1]);
        assert!(get_no_zero_integers(11) == vec![2, 9]);
        assert!(get_no_zero_integers(10000) == vec![1, 9999]);
        assert!(get_no_zero_integers(1010) == vec![11, 999]);
    }
}
