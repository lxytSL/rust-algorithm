// 3005. 最大频率元素计数

use std::collections::HashMap;

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut mp = HashMap::new();
    let mut max_num = 0;
    for num in nums {
        *mp.entry(num).or_insert(0) += 1;
        if mp.get(&num) > Some(&max_num) {
            max_num = *mp.get(&num).unwrap()
        }
    }
    // max_num的数
    let mut count = 0;
    for item in mp.values() {
        if *item == max_num {
            count += 1;
        }
    }
    count * max_num
}
