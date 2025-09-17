// 2197. 替换数组中的非互质数

pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();

    for mut num in nums {
        while let Some(&last) = ans.last() {
            let g = gcd(last, num);
            if g > 1 {
                // 计算最小公倍数
                num = last / g * num ;
                ans.pop();
            } else {
                break;
            }
        }
        ans.push(num);
    }
    ans
}

// 计算两个数的最小公倍数
fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

// 计算两个数的最大公约数
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn replace_non_coprimes_test() {
        assert!(replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2]) == vec![12, 7, 6]);
    }
    #[test]
    fn replace_non_coprimes_test2() {
        assert!(
            replace_non_coprimes(vec![
                31, 97561, 97561, 97561, 97561, 97561, 97561, 97561, 97561
            ]) == vec![31, 97561]
        );
    }
}
