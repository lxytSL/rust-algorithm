// 2785. 将字符串中的元音字母排序

pub fn sort_vowels(s: String) -> String {
    let mut vowels = Vec::new();
    let vowels_tag = vec!['a', 'e', 'i', 'o', 'u'];
    for c in s.chars() {
        let low_c = c.to_lowercase().next().unwrap();
        if vowels_tag.contains(&low_c) {
            vowels.push(c);
        }
    }

    vowels.sort();
    dbg!(&vowels);
    let mut ans = String::new();
    for c in s.chars() {
        let low_c = c.to_lowercase().next().unwrap();
        if vowels_tag.contains(&low_c) {
            // 弹出第一个元素
            ans.push(vowels.remove(0));
        } else {
            ans.push(c);
        }
    }
    dbg!(&ans);
    ans
}

mod tests {
    use super::*;
    #[test]
    fn sort_vowels_test() {
        assert!(sort_vowels("hello".to_string()) == "hello");
    }
    #[test]
    fn sort_vowels_test2() {
        assert!(sort_vowels("lEetcOde".to_string()) == "lEOtcede");
    }
}
