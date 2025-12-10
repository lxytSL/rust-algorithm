use std::collections::HashMap;

pub fn special_triplets(nums: Vec<i32>) -> i32 {
    let mode = 1000000007;
    let mut mp_index = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        mp_index.entry(v).or_insert(Vec::new()).push(i);
    }
    let mut ans = 0;
    for (i, v) in nums.iter().enumerate() {
        let tag = *v * 2;
        if let Some(idx) = mp_index.get(&tag) {
            dbg!(idx);
            // 查找i在idx中的位置
            if idx.len() >= 2 {
                match idx.binary_search(&i) {
                    Ok(search_idx) => {
                        ans += (search_idx * (idx.len() - search_idx - 1) % mode) % mode;
                    }
                    Err(search_idx) => {
                        ans += (search_idx * (idx.len() - search_idx) % mode) % mode;
                    }
                }
            }
        }
    }
    ans as i32
}

mod test {
    use super::*;
    #[test]
    fn test_special_triplets() {
        assert!(special_triplets(vec![0, 1, 0, 0]) == 1);
    }
}
