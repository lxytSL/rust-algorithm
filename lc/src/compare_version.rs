// 165. 比较版本号

pub fn compare_version(version1: String, version2: String) -> i32 {
    let version1_part: Vec<&str> = version1.split(".").collect();
    let version2_part: Vec<&str> = version2.split('.').collect();
    let len_max = if version1_part.len() > version2_part.len() {
        version1_part.len()
    } else {
        version2_part.len()
    };
    for part_idx in 0..len_max {
        let version1_sub = if part_idx >= version1_part.len() {
            0
        } else {
            version1_part[part_idx]
                .trim_start_matches('0')
                .parse::<i32>()
                .unwrap_or(0)
        };
        let version2_sub = if part_idx >= version2_part.len() {
            0
        } else {
            version2_part[part_idx]
                .trim_start_matches('0')
                .parse::<i32>()
                .unwrap_or(0)
        };
        if version1_sub > version2_sub {
            return 1;
        }
        if version1_sub < version2_sub {
            return -1;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn compare_version_test() {
        assert!(compare_version("1.0.0".to_string(), "1.0.0".to_string()) == 0);
        assert!(compare_version("1.2".to_string(), "1.10".to_string()) == -1);
        assert!(compare_version("1.01".to_string(), "1.001".to_string()) == 0);
    }
}
