pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    // todo!()

    // 动态规划（Dynamic Programming）算法
    // 这里不会，是AI教的
    // 钱币面值
    let coins = vec![1,2,5,10,20,50,100];
    // 初始值为 amount + 1 , 用于存储凑出每个金额所需的最少硬币数量
    let mut dp = vec![amount + 1; amount as usize + 1];
    // 初始值为 0, 因为凑出金额为 0 所需的最少硬币数量为 0
    dp[0] = 0;
    // 遍历从 1 到 amount 的每个金额
    for i in 1..=amount as usize {
        // 遍历所有硬币面值
        for &coin in coins.iter() {
            // 如果当前硬币面值小于等于当前金额
            if coin as usize <= i {
                // 更新凑出当前金额所需的最少硬币数量
                dp[i as usize] = dp[i as usize].min(dp[i - coin as usize] + 1);
            }
        }
    }
    // 返回凑出金额 amount 所需的最少硬币数量
    dp[amount as usize]
}
