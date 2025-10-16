// 2273. 移除字母异位词后的结果数组
pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    let mut ans = vec![words[0].clone()];
    let mut flag = [0u8; 26];
    for b in words[0].bytes() {
        let idx = b - 'a' as u8;
        flag[idx as usize] += 1;
    }
    for item in words[1..].iter() {
        let mut target = [0u8; 26];
        for b in item.bytes() {
            let idx = b - 'a' as u8;
            target[idx as usize] += 1;
        }
        if !flag.eq(&target) {
            ans.push(item.clone());
            flag = target;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_anagrams_test() {
        assert!(
            remove_anagrams(vec![
                "abba".to_string(),
                "baba".to_string(),
                "bbaa".to_string(),
                "cd".to_string(),
            ]) == vec!["abba".to_string(), "cd".to_string()]
        );
    }
}
