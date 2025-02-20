pub fn goldbach_conjecture() -> String {
    // todo!()
    let mut non_writable_numbers = Vec::new();
    let mut num = 9; // 从最小的奇合数开始
    while non_writable_numbers.len() < 2 {
        if!is_prime(num) && num % 2 != 0 &&!can_be_written(num) {
            non_writable_numbers.push(num);
        }
        num += 2; // 只检查奇数
    }
    format!("{},{}", non_writable_numbers[0], non_writable_numbers[1])
}

// 判断一个数是否为素数
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 判断一个奇合数是否可以写成一个素数和一个平方的两倍之和
fn can_be_written(n: u32) -> bool {
    for i in 1.. {
        let square = 2 * i * i;
        if square >= n {
            break;
        }
        let prime = n - square;
        if is_prime(prime) {
            return true;
        }
    }
    false
}