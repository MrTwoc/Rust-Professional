pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    // todo!()

    // 定义一个数组来存储测试用例和预期结果
    let mut src:Vec<&str> = Vec::new();
    // 获取传入的源数值，用split方法分割出数字和进制
    let src_num:Vec<&str> = num_str.split('(').collect();
    // 将数字放入数组
    src.push(src_num[0]);
    // 获取源进制
    let src_base:Vec<&str> = src_num[1].split(')').collect();
    // 将进制放入数组
    src.push(src_base[0]);
    // println!("{}进制的数字{}",src[1],src[0]);
    // 将进制转换为u32
    let src_base_u32 = src[1].parse::<u32>().unwrap();
    // 将数字转换为u32
    let num:u32 = u32::from_str_radix(src[0],src_base_u32).expect("不是有效数字");
    // 存储结果
    let mut result = String::new();
    // n：待转换的数字，定义一个可变的 'u64' 后续会继续操作
    let mut n = num;

    // 如果 'n' = 0 ，则直接返回 "0"
    if n == 0 {
        return "0".to_string();
    }

    // 将数字转换为目标进制:
    // 当 'n' 除以 'to_base' 时，将十进制数 'n' 转换为目标进制的字符串
    while n > 0 {
        // 计算 'n' 除以 'to_base' 的余数 'digit'， 得到当前位的数字，将结果转换为 'u32' 类型
        let digit = (n % to_base) as u32;
        // 定义一个可变的字符串变量 'char'，用于存储当前位的字符表示
        let char = if digit < 10 {
            // 如果当前位的数字小于10，将其转换为对应的ASCII字符(数字字符 '0' - '9' )
            // 例如，数字 0 对应的 ASCII 码是 48（b'0'），所以 digit 加上 b'0' 就得到对应的字符
            (digit as u8 + b'0') as char
        } else {
            // 如果当前位的数字 >= 0 ,将其转换为对应的字母字符 (小写字母 'a' - 'f')
            // 例如，数字 10 对应的字母是 'a'，所以 digit 减去 10 再加上 b'a' 就得到对应的字母字符
            (digit as u8 - 10 + b'a') as char
        };
        // 将当前位对应的字符插入到 'result' 字符串的开头
        result.insert(0, char);
        // 将 'n' 除以 'to_base' 的商赋值给 'n'，继续进行下一位的转换
        n /= to_base;
    }
    result
}
