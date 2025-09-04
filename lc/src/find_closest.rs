// 3516. 找到最近的人
pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    if (x - z).abs() < (y - z).abs() {
        1
    } else if (x - z).abs() > (y - z).abs() {
        2
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_closest_test() {
        assert!(find_closest(2, 7, 4) == 1);
        assert!(find_closest(2, 5, 6) == 2);
        assert!(find_closest(1, 5, 3) == 0);
    }
}
