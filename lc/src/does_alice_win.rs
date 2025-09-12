// 3227. 字符串元音游戏

pub fn does_alice_win(s: String) -> bool {
    s.chars().any(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn does_alice_win_test() {
        assert!(does_alice_win("leetcoder".to_string()) == true);
        assert!(does_alice_win("bbcd".to_string()) == false);
    }
}
