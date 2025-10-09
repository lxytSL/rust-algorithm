// 3494. 酿造药水需要的最少总时间
pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
    // finish 完成时间
    let l = skill.len();
    let ml = mana.len();
    let mut finish = vec![0; l];
    // 第一轮
    finish[0] = mana[0] as i64 * skill[0] as i64;
    for (i, &v) in skill.iter().enumerate().skip(1) {
        finish[i] = finish[i - 1] + v as i64 * mana[0] as i64;
    }
    dbg!(&finish);
    for i in 1..ml {
        // 计算下一轮的开始时间
        // 分别计算每个巫师最晚的开始时间
        // 每个巫师的最晚开始时间为
        // 上一轮完成时间finish[i]-前面巫师所需要的时间
        // 取最大值就是开始时间
        let mut start_time = finish[0];
        let mut used_time = skill[0] as i64 * mana[i] as i64;
        for j in 1..l {
            start_time = std::cmp::max(start_time, finish[j] - used_time);
            used_time += (mana[i] as i64 * skill[j] as i64);
        }
        dbg!(&start_time);
        // 开始时间计算出来后，计算本次每个巫师的完成时间
        for k in 0..l {
            finish[k] = start_time + mana[i] as i64 * skill[k] as i64;
            start_time = finish[k];
        }
        dbg!(&finish);
    }
    finish[l - 1]
}

mod test {
    use super::*;
    #[test]
    fn test_min_time() {
        assert!(min_time(vec![3, 5, 3, 9], vec![1, 10, 7, 3]) == 293);
    }
}
