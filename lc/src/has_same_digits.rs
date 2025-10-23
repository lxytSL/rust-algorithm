// 3461. 判断操作后字符串中的数字是否相等 I
pub fn has_same_digits(s: String) -> bool {
    let l = s.len();
    let mut dig: Vec<u8> = s.as_bytes().to_vec();
    for round in 0..l - 2 {
        for i in 0..l - round - 1 {
            let digit1 = (dig[i] - b'0') as u8;
            let digit2 = (dig[i + 1] - b'0') as u8;
            dig[i] = (digit1 + digit2) % 10;
        }
    }
    dig[0] == dig[1]
}
