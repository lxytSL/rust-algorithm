// 1935. 可以输入的最大单词数

pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let words = text.split_whitespace().collect::<Vec<&str>>();
    let mut count = 0;
    for word in words {
        let mut can_input = true;
        // broken_letters中的字母不能出现在word中
        // 只要存在一个字母出现在word中，就认为不能输入
        for c in word.chars() {
            if broken_letters.contains(c) {
                can_input = false;
                break;
            }
        }
        if can_input {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn can_be_typed_words_test() {
        assert!(can_be_typed_words("hello world".to_string(), "ad".to_string()) == 1);
    }
}
