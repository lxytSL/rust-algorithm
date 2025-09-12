// 1733. 需要教语言的最少人数

use std::collections::{HashMap, HashSet};

pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    // 用来统计不能互相沟通的人
    let mut mp = HashSet::new();
    for fri in friendships {
        let mut set = HashSet::new();
        let mut comm = false;
        // 第一个人掌握的语言
        for &lan in &languages[(fri[0] - 1) as usize] {
            set.insert(lan);
        }
        // 第二个人掌握的语言
        for &lan in &languages[(fri[1] - 1) as usize] {
            if set.contains(&lan) {
                comm = true;
                break;
            }
        }
        // 如果不能沟通
        if !comm {
            mp.insert(fri[0] - 1);
            mp.insert(fri[1] - 1);
        }
    }
    let mut max_lan_cnt = 0;
    // 统计不能交流的人中，掌握最多的一门语言的数量
    let mut lan_cnt = HashMap::new();
    for &p in &mp {
        for &lan in &languages[p as usize] {
            *lan_cnt.entry(lan).or_insert(0) += 1;
            max_lan_cnt = max_lan_cnt.max(*lan_cnt.get(&lan).unwrap());
        }
    }
    (mp.len() - max_lan_cnt) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_teachings_test() {
        assert!(
            minimum_teachings(
                3,
                vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
                vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]]
            ) == 2
        );
    }
}
