pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    // todo!()
    if n <= 1 {
        return 0.0
    }
    if n >= 365 {
        return 1.0
    }

    // 存储生日的不同概率
    let mut probability = 1.0;

    /*
    假设 n = 3：
     
    当 i = 0 时，(365.0 - 0 as f64) / 365.0 = 1.0，
    此时 probability = 1.0 * 1.0 = 1.0。

    当 i = 1 时，(365.0 - 1 as f64) / 365.0 = 364.0 / 365.0，
    此时 probability = 1.0 * (364.0 / 365.0)。

    当 i = 2 时，(365.0 - 2 as f64) / 365.0 = 363.0 / 365.0，
    此时 probability = 1.0 * (364.0 / 365.0) * (363.0 / 365.0)。
    */
    for i in 0..n {
        probability *= (365.0 - i as f64) / 365.0;
    }

    /*
    当 n = 3 时，363.0 / 365.0 = 0.9973
    1.0 - 0.9973 = 0.0027
    所以，在 3 个人中，有两个人在同一天过生日的概率是 0.0027
    */
    1.0 - probability
}
