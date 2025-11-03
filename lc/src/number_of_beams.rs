// 2125. 银行中的激光束数量

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut pre_device = 0;
    let mut ans = 0;
    for s in bank {
        let cur_device = s.as_bytes().iter().filter(|&&b| b == b'1').count();
        if cur_device == 0 {
            continue;
        }
        ans += pre_device * cur_device;
        pre_device = cur_device;
    }
    ans as i32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_number_of_beams() {
        assert!(
            number_of_beams(vec![
                "011001".to_string(),
                "000000".to_string(),
                "010100".to_string(),
                "001000".to_string()
            ]) == 8
        );
    }
}
