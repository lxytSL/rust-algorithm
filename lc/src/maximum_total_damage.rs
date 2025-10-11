// 3186. 施咒的最大总伤害

use std::collections::BTreeMap;

pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
    let mut count = BTreeMap::new();
    for p in power {
        *count.entry(p).or_insert(0) += 1;
    }
    let mut vec = vec![(-1_000_000_000i32, 0i32)];
    for (k, v) in count {
        vec.push((k, v));
    }
    let n = vec.len();
    let mut f = vec![0i64; n];
    let mut mx = 0i64;
    let mut ans = 0i64;
    let mut j = 1usize;
    for i in 1..n {
        while j < i && vec[j].0 < vec[i].0 - 2 {
            if f[j] > mx {
                mx = f[j];
            }
            j += 1;
        }
        f[i] = mx + vec[i].0 as i64 * vec[i].1 as i64;
        dbg!(&f);
        if f[i] > ans {
            ans = f[i];
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_maximum_total_damage() {
        assert!(maximum_total_damage(vec![1, 1, 3, 4, 3, 3, 3]) == 12);
    }
}
