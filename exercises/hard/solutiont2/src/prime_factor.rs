/*
    这里借鉴的其他同学的代码，用ai理解的
    另外用了ai优化一下，但是ai的代码有问题，我自己改了一下
*/
//! 质因数分解模块
//!
//! 本模块提供了一个高效的算法来寻找给定数的最大质因数。
//! 该算法结合了埃拉托斯特尼筛法和优化的因数检查方法，适用于处理大数。

/// 寻找给定数的最大质因数
///
/// # 参数
/// * `number` - 需要分解的正整数，类型为u128
///
/// # 返回值
/// 返回`number`的最大质因数，类型为u128
///
/// # 示例
/// ```
/// let max_prime = find_max_prime_factor(13195);
/// assert_eq!(max_prime, 29);
/// ```
///
/// # 算法说明
/// 1. 首先处理2和3的因子，因为它们是最小的质数且可以快速处理
/// 2. 使用埃拉托斯特尼筛法生成一个素数表，用于检查小素数因子
/// 3. 对于大素数因子，使用优化的步长检查方法（检查6k±1形式的数）
/// 4. 最后处理可能剩余的大素数
pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut number = number;
    let mut max_prime = 0;
    
    // 处理2和3的因子
    while number % 2 == 0 {
        max_prime = 2;
        number /= 2;
    }
    while number % 3 == 0 {
        max_prime = 3;
        number /= 3;
    }

    // 生成素数表
    let limit = number.isqrt().min(1_000_000) as u64;
    let small_primes = sieve_of_eratosthenes(limit);

    // 检查小素数因子
    for &prime in &small_primes {
        if number == 1 {
            break;
        }
        while number % prime as u128 == 0 {
            max_prime = prime as u128;
            number /= prime as u128;
        }
    }

    // 确定大素数因子的起始检查点
    // 如果number的平方根大于素数表中最后一个素数，则从最后一个素数的平方开始检查
    // 否则从最后一个素数开始检查
    let mut factor = if number.isqrt() > *small_primes.last().unwrap() as u128 {
        small_primes.last().unwrap().pow(2) as u128
    } else {
        *small_primes.last().unwrap() as u128
    };

    while factor * factor <= number {
        if number % factor == 0 {
            max_prime = factor;
            number /= factor;
            continue;
        }
        if number % (factor + 2) == 0 {
            max_prime = factor + 2;
            number /= factor + 2;
            continue;
        }
        if number % (factor + 4) == 0 {
            max_prime = factor + 4;
            number /= factor + 4;
            continue;
        }
        if number % (factor + 6) == 0 {
            max_prime = factor + 6;
            number /= factor + 6;
            continue;
        }
        factor += 8;
    }

    // 处理最后的素数
    if number > 2 {
        max_prime = number;
    }
    max_prime
}

/// 用埃拉托斯特尼筛法生成素数表
///
/// # 参数
/// * `limit` - 生成素数的上限，类型为u64
///
/// # 返回值
/// 返回一个包含所有小于等于`limit`的素数的Vec<u64>
///
/// # 算法说明
/// 1. 初始化一个布尔数组，标记所有数为素数
/// 2. 从2开始，将每个素数的倍数标记为非素数
/// 3. 最后收集所有标记为素数的数
///
/// # 复杂度
/// 时间复杂度：O(n log log n)
/// 空间复杂度：O(n)
fn sieve_of_eratosthenes(limit: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=limit.isqrt() {
        if is_prime[i as usize] {
            for j in (i * i..=limit).step_by(i.try_into().unwrap()) {
                is_prime[j as usize] = false;
            }
        }
    }
    
    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &prime)| prime)
        .map(|(index, _)| index as u64)
        .collect()
}
