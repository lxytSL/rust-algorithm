// 3147. 从魔法师身上吸取的最大能量

pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    // 倒叙遍历终点
    let l = energy.len();
    let left = l - k as usize;
    let mut ans = std::i32::MIN;
    for i in left..l {
        let mut j = i as i32;
        let mut sum = 0;
        while j >= 0 {
            sum += energy[j as usize];
            ans = std::cmp::max(ans, sum);
            j -= k;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_maximum_energy() {
        assert!(maximum_energy(vec![8,-5], 1) == 3);
    }
}
