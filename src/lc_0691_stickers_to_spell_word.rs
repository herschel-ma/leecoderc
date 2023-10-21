use std::collections::HashMap;
/// Solution function 题解
/// 这道题给了我们N个贴片，每个贴片上有一个小写字母的单词，给了我们一个
/// 目标单词 target, 让我们通过剪下贴片能拼出目标值 target, 如果发现不行，
/// 有网友留⾔提示说是多重背包问题，然后去论坛上看大神们的解法，果然都是
/// 用 DP 做的，之前曾有网友推荐过一人“背包九讲” 的贴子，大概扫过几眼，真
/// 是叼到飞起啊，博主希望有时间也能总结一下。先来看看这道题吧，既然是用
/// DP 来做，那么就需要用 dp 数组了，我们用一个一维的 dp 数组，其中 dp[i]
/// 表示组成第 i 个子集合所需要的最少的 sticker 的个数，注意这里是子集合，
/// 而不是子串。长度为 n 的字符串共有2的n次方个集合，比如字符串 "ab"， 就
/// 有 4 个子集合，分别是"", "a", "b", "ab"。字符串 "abc" 就有 8 个子集
/// 合，如果我们用 0 到 7 来分别对应其子集合，就有：
///     abc     subset
/// 0   000     ""
/// 1   001     c
/// 2   010     b
/// 3   100     a
/// 4   101     ac
/// 5   110     ab
/// 6   111     abc
/// 7   011     bc
/// 我们可以看到0到7的二进制的每一位上的0和1就相当于 mask,是1的话就选上该
/// 位对应的字母，000就表示都不选，就是空集，111就表示都选，就是 abc, 那
/// 么只要我们将所有子集合的dp都算出来，最后返回数组的最后一个位置上的数
/// 字，就是和目标子串相等的子集合啦。我们以下面这个简单的例子来讲解：
///
/// `stickers = vec![String::from("ab"), String:from("bc"), String::from("ac")];`
/// `target = String::from("abc")`
///
/// 之前说了abc的共有8个子集合，所以dp数组的长度为8,除了dp[0]初始化为0之外，
/// 其余的都初始化为 INT_MAX，然后我们开始逐个更新 dp 数组的值，我们的目标
/// 是从 sticker 中取出字符，来拼出子集合，所以如果当前遍历的 dp 值仍为
/// INT_MAX 的话，说明了该子集合无法被拼出来，自然我们也无法再其基础上去拼
/// 别的子集合了，真接跳过。否则我们就来遍历所有的 sticker, 让变量 cur 等
/// 于 i, 说明此时是在第 i 个子集合的基础上去 reach 其他的子集合，我们遍历
/// 当前 sticker 的每一个字母，对于sticker的每一个字母，我们都扫描一遍target
/// 的所有字符，如果 target 里有这个字符，且该字符未出现在 cur 位置的子集
/// 合中，则将这个字符加入到子集合中。什么意思呢？比如当前我们的 cur 是 3，
/// 二进制为 011, 对应的子集合为 "bc", 此时如果我们遍历的 sticker 是 "ab",
/// 那么遇到"a"时，我们知道 target 中是有 "a" 的，然后我们看"bc"中包不包括
/// "a", 判断方法是看 (cur>>k) & 1 是否为 0，这可以乍看上去不太好理解，其
/// 实不难想，因为我们的子集合是跟二进皮对应的，"bc" 就对应着011，第一个0
/// 就表示"a"缺失，所以我们想看哪个字符，就提取出该字符的二进制位，提取方法
/// 就是 cur>>k, 表示 cur 向右移动 k 位，太长不打了！！！
///
/// Args:
///     stickers: Vec<String>: 贴纸
///     target: String: 要拼写出的单词
/// Returns:
///     i32: 拼写出单词需要的最少的贴纸数量
pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    let n = target.len();
    let mut dp = vec![std::i32::MAX; 1 << n];
    dp[0] = 0;
    let cnts: Vec<HashMap<char, i32>> = stickers
        .iter()
        .map(|s| {
            let mut cnt = HashMap::new();
            for ch in s.chars() {
                *cnt.entry(ch).or_insert(0) += 1;
            }
            cnt
        })
        .collect();
    for state in 0..(1 << n) {
        if dp[state] == std::i32::MAX {
            continue;
        }
        for cnt in &cnts {
            let mut now = state;
            let mut new_count = cnt.clone();
            for (i, ch) in target.chars().enumerate() {
                if now & (1 << i) != 0 {
                    continue;
                }
                if let Some(x) = new_count.get_mut(&ch) {
                    if *x > 0 {
                        *x -= 1;
                        now |= 1 << i;
                    }
                }
            }
            dp[now] = dp[now].min(dp[state] + 1);
        }
    }
    if dp[(1 << n) - 1] == std::i32::MAX {
        -1
    } else {
        dp[(1 << n) - 1]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_min_stickers() {
        use super::*;
        assert_eq!(
            min_stickers(
                vec![
                    String::from("with"),
                    String::from("example"),
                    String::from("science")
                ],
                String::from("thehat")
            ),
            3
        )
    }

    #[test]
    fn test_min_stickers2() {
        use super::*;
        assert_eq!(
            min_stickers(
                vec![String::from("notice"), String::from("possible")],
                String::from("basicbasic")
            ),
            -1
        )
    }
}
