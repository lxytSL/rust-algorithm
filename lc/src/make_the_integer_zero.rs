// 2749. 得到整数零需要执行的最少操作数

pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
    let mut k: i64 = 0;
    loop {
        // 减去k的num2之后剩余的值
        let x = num1 as i64 - k * num2 as i64;
        // 判断剩下的值是否是k * 2的次幂
        if x < k {
            //  必然不能
            return -1;
        }
        // x = k * 2 ^ i
        // 一旦出现k > x中1的个数，都可以操作
        // 又由于k增加，x是减少的
        // 所以出k > x中的1就是最小值
        if k >= x.count_ones() as i64 {
            return k as i32;
        }
        k += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert!(make_the_integer_zero(112577768, -501662198) == 16);
        assert!(make_the_integer_zero(3, -2) == 3);
        assert!(make_the_integer_zero(5, 7) == -1);
    }
}
