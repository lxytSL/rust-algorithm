// 1039. 多边形三角剖分的最低得分

pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut memo = vec![vec![-1; n]; n];

    fn dfs(i: usize, j: usize, values: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if i + 1 == j {
            return 0; // 两点之间没法组成三角形
        }
        if memo[i][j] != -1 {
            return memo[i][j];
        }
        let mut res = i32::MAX;
        for k in i + 1..j {
            let cost = dfs(i, k, values, memo)
                + dfs(k, j, values, memo)
                + values[i] * values[k] * values[j];
            res = res.min(cost);
        }
        memo[i][j] = res;
        res
    }

    dfs(0, n - 1, &values, &mut memo)
}
