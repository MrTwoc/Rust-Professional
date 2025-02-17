pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    // todo!()
    let mut src:Vec<&str> = Vec::new();
    // 获取源数值
    let src_num:Vec<&str> = num_str.split('(').collect();
    src.push(src_num[0]);
    // 获取源进制
    let src_base:Vec<&str> = src_num[1].split(')').collect();
    src.push(src_base[0]);
    // println!("{}进制的数字{}",src[1],src[0]);
    let src_base_u32 = src[1].parse::<u32>().unwrap();
    let num:u32 = u32::from_str_radix(src[0],src_base_u32).expect("不是有效数字");
    let mut result = String::new();
    let mut n = num;

    if n == 0 {
        return "0".to_string();
    }

    while n > 0 {
        let digit = (n % to_base) as u32;
        let char = if digit < 10 {
            (digit as u8 + b'0') as char
        } else {
            (digit as u8 - 10 + b'a') as char
        };
        result.insert(0, char);
        n /= to_base;
    }
    result

    
}
