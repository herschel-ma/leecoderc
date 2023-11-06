use std::iter::Iterator;

pub fn paint_walls_error(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    // 雇佣一位花费为 cost[i] 的粉刷匠可以工作 time[i] 的时间，也就是
    // 可以白嫖掉 time[i] 堵墙。换言之，雇佣这位粉刷匠相当于使用 cost[i]
    // 来刷掉 time[i] + 1 堵墙。那么到了这里，问题就可以转化成背成问题了，
    // 不过与常规背包问题不同的是，本题中的物品大小是可以超出背包大小的，
    // 因此需要对这些情况进行特殊处理。

    // 令f[i][j] 表示前i种付费方案，刷掉j堵墙所需的最小花销，那么对于f[i][j]，
    // 首先可以直拉跳过cost[i]这种方案，于是可以初始化f[i][j] = f[i-1][j].
    // 接下来考虑time[i]+1与j的关系。若time[i] + 1 >= j, 此时只需釆用cost[i]
    // 这一个方案就可以刷掉全部的j堵墙，于是就有f[i][j] = min(f[i][j], cost[i])
    // 否则就有f[i][j] = min(f[i][j], f[i-1][j-time[i]-1] + cost[i])。
    // 另外，注意到f的每一轮只与上一轮有关，因此可以通过倒序计算省略这一维。

    let n = cost.len();
    let mut f = vec![None; n + 1];
    f[0] = Some(0);
    let mut _res: Option<i32> = None;

    for (c, t) in cost.iter().zip(time.iter()) {
        // println!("{}-{}", c, t);
        if (t + 1) as usize >= n {
            if f[n].is_none() || f[n].unwrap() > *c {
                f[n] = Some(*c)
            }
        } else {
            for index in (*t..n as i32).rev() {
                if f[(index - *t - 1).max(0) as usize].is_some()
                    && (f[index as usize].is_none()
                        || f[index as usize] > f[(index - *t - 1).max(0) as usize])
                {
                    f[index as usize] = Some(f[(index - *t - 1).max(0) as usize].unwrap() + *c)
                }
            }

            for i in (0..*t).rev() {
                if f[i as usize].is_none() || f[i as usize].unwrap() > *c {
                    f[i as usize] = Some(*c)
                }
            }
        }
    }

    f[n].unwrap_or(0)
}

pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 1..n + 1 {
        dp[n][i] = 1_000_000_000;
    }

    for i in (0..n).rev() {
        for need_to_take in 1..n + 1 {
            let take_yes = cost[i] + dp[i + 1][0.max(need_to_take as i32 - 1 - time[i]) as usize];
            let take_no = dp[i + 1][need_to_take];
            dp[i][need_to_take] = take_yes.min(take_no);
        }
    }
    dp[0][n]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        use super::paint_walls;
        let cost = vec![1, 2, 3, 2];
        let time = vec![1, 2, 3, 2];
        assert_eq!(paint_walls(cost, time), 3);
    }

    #[test]
    fn test_case_2() {
        use super::paint_walls;
        let cost = vec![2, 3, 4, 2];
        let time = vec![1, 1, 1, 1];
        assert_eq!(paint_walls(cost, time), 4)
    }
}
