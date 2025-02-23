use std::collections::{HashMap, HashSet};
// 生成了详细的注释，方便复习 

/// 判断是否为闰年
/// 原理：根据格里高利历法规则：
/// 1. 能被4整除但不能被100整除的年份是闰年
/// 2. 能被400整除的年份也是闰年
fn is_leapyear(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// 将字符串转换为无符号整数
/// 实现原理：
/// 1. 逐个字符处理 ASCII 码
/// 2. 通过减去 '0' 的 ASCII 码得到数字值
/// 3. 按十进制位权累加
fn str_to_num(s: &str) -> u32 {
    let tmp = s.as_bytes();
    let mut number = 0;
    for num in tmp.iter() {
        number = number * 10 + (*num - b'0') as u32
    }
    number
}

/// 计算指定日期的星期几（返回 1-7 对应周一到周日）
/// 使用蔡勒公式实现，步骤：
/// 1. 调整年月参数（1月2月视为前一年的13月14月）
/// 2. 计算公式参数：
///    c = 世纪数（年份前两位）
///    y = 年份后两位
///    m = 调整后的月份
///    d = 日期
/// 3. 计算结果并取模转换
fn date_to_week_day(year: u32, month: u32, day: u32) -> u32 {
    let c = (year / 100) as i32;
    let mut y = (year % 100) as i32;
    let mut m: i32 = 0;
    let d: i32 = day as i32;
    if month == 1 || month == 2 {
        m = month as i32 + 12;
        y -= 1;
    } else {
        m = month as i32;
    };
    let mut h = (y + y/4 + c/4 - 2*c + (13*(m as i32 + 1)/5 + d - 1)) % 7;
    if h == 0 {
        h = 7;
    }
    h as u32
}

/// 计算指定日期在一年中的周数（ISO 8601 标准）
/// 实现步骤：
/// 1. 确定当年1月1日的星期几
/// 2. 处理跨年周的特殊情况：
///    - 如果1月1日是周五、周六或周日，则第一周可能属于上一年
///    - 如果12月31日属于下一年的第一周，则周数返回1
/// 3. 根据已过天数计算周数
fn date_to_week_count(year: u32, month: u32, day: u32) -> u32 {
    let mut week_day = date_to_week_day(year, 1, 1);
    if week_day == 7 {
        week_day = 0;
    }
    let (days, _) = count_days(year, month, day);
    
    let mut ans = 0;
    if week_day > 4 {
        if day <= 7-week_day+1 {
            ans = date_to_week_count(year-1, 12, 31);
        } else {
            ans = (days + week_day - 3) / 7 + 1;
        }
    } else {
        ans = (days + week_day - 2) / 7 + 1;
    }

    if month == 12 {
        let cur_year_last_week_day = date_to_week_day(year, 12, 31);
        if cur_year_last_week_day < 4 && 31-day < cur_year_last_week_day {
            1
        } else {
            ans
        }
    } else {
        ans
    }
}

/// 计算日期在年内的天数分布
/// 返回元组：(当年已过天数, 当年剩余天数)
/// 实现步骤：
/// 1. 根据是否闰年初始化每月天数表
/// 2. 累加当前月份之前所有月份的天数
/// 3. 加上当前月份的天数得到已过天数
/// 4. 用全年天数减去已过天数得到剩余天数
fn count_days(year: u32, month: u32, day: u32) -> (u32, u32) {
    let mut ans_0 = 0;
    let mut ans_1 = 0;
    let mut month2days: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leapyear(year) {
        month2days[2] = 29;
    }
    let is_leapyear = is_leapyear(year);
    for m in 1..month {
        ans_0 += month2days[m as usize];
    }
    ans_0 += day;

    if is_leapyear {
        ans_1 = 366 - ans_0;
    } else {
        ans_1 = 365 - ans_0;
    }

    (ans_0, ans_1)
}

/// 计算两个日期之间的天数差
/// 处理逻辑：
/// 1. 同年日期：直接计算两个日期的年内天数差
/// 2. 跨年日期：计算第一个日期到年底的天数 + 第二个日期在次年的天数
fn compute_diff_of_date(year0: u32, month0: u32, day0: u32, year1: u32, month1: u32, day1: u32) -> u32 {
    if year0 == year1 {
        let (days_0, _) = count_days(year0, month0, day0);
        let (days_1, _) = count_days(year1, month1, day1);
        days_0 - days_1
    } else {
        let (days_0, _) = count_days(year0, month0, day0);
        let (_, days_1) = count_days(year1, month1, day1);
        days_0 + days_1
    }
}

/// 计算距离最近春节的天数
/// 参数 date2newyear 应包含春节日期映射表（年份 -> (月, 日)）
/// 实现逻辑：
/// 1. 确定当前日期所属的农历年对应的春节日期
/// 2. 如果当前日期已过当年春节，则计算到下一年春节的天数
fn date_to_newyear(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> u32 {
    let mut newyear_year = year;
    let mut newyear_month = 0;
    let mut newyear_day = 0;
    (newyear_month, newyear_day) = *date2newyear.get(&year).expect("date_to_newyear 错误");
    if month > newyear_month || (month == newyear_month && day > newyear_day) {
        (newyear_month, newyear_day) = *date2newyear.get(&(year+1)).expect("date_to_newyear 错误");
        newyear_year = year + 1;
    }
    compute_diff_of_date(newyear_year, newyear_month, newyear_day, year, month, day)
}

/// 获取下一天的日期
/// 处理特殊日期：
/// 1. 月末：跳转到下个月首日
/// 2. 年末：跳转到次年1月1日
/// 3. 闰年二月末的特殊处理
fn next_day(year: u32, month: u32, day: u32) -> (u32, u32, u32) {
    let mut month2days: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leapyear(year) {
        month2days[2] = 29;
    }
    if month2days[month as usize] == day {
        if month == 12 {
            (year+1, 1, 1)
        } else {
            (year, month+1, 1)
        }
    } else {
        (year, month, day+1)
    }
}

/// 判断指定日期是否为节假日（含周末调整）
/// 包含的节假日：
/// 1. 元旦（1月1日）
/// 2. 春节（假期包含除夕和7天春节假期）
/// 3. 劳动节（5月1日-5月5日）
/// 4. 周末（周六、周日）
/// 注意：需要传入春节日期映射表
fn is_holiday(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> bool {
    let mut month2days: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leapyear(year) {
        month2days[2] = 29;
    }
    let mut holidays: HashSet<(u32, u32)> = HashSet::new();

    holidays.insert((1, 1));

    let (mut newyear_month, mut newyear_day) = *date2newyear.get(&year).expect("date_to_newyear 错误");
    if newyear_day != 1 {
        holidays.insert((newyear_month, newyear_day-1));
    } else {
        holidays.insert((newyear_month-1, month2days[newyear_month as usize - 1]));
    }
    for _ in 0..7 {
        holidays.insert((newyear_month, newyear_day));
        (_, newyear_month, newyear_day) = next_day(year, newyear_month, newyear_day);
    }

    holidays.insert((5, 1));
    holidays.insert((5, 2));
    holidays.insert((5, 3));
    holidays.insert((5, 4));
    holidays.insert((5, 5));
    if holidays.contains(&(month, day)) {
        true
    } else {
        let week_day = date_to_week_day(year, month, day);
        match week_day {
            1..=5 => false,
            _ => true,
        }
    }
}

/// 计算到下一个交易日的天数差
/// 实现逻辑：
/// 1. 循环查找下一个非假日的日期
/// 2. 使用 compute_diff_of_date 计算天数差
/// 注意：跳过所有节假日和周末
fn date_to_trading_days(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> u32 {
    let mut trading_day = next_day(year, month, day);
    while is_holiday(trading_day.0, trading_day.1, trading_day.2, date2newyear) {
        trading_day = next_day(trading_day.0, trading_day.1, trading_day.2);
    }
    compute_diff_of_date(trading_day.0, trading_day.1, trading_day.2, year, month, day) - 1
}

pub fn time_info(time: &str) -> String {
    let tmp: Vec<&str> = time.split('-').collect();
    let year = str_to_num(tmp[0]);
    let month = str_to_num(tmp[1]);
    let day = str_to_num(tmp[2]);
    
    let week_count = date_to_week_count(year, month, day);
    let week_day = date_to_week_day(year, month, day);

    let (days_0, days_1) = count_days(year, month, day);

    let mut date2newyear = HashMap::new();
    date2newyear.insert(2025, (1, 29));
    date2newyear.insert(2026, (2, 17));
    let days_2 = date_to_newyear(year, month, day, &date2newyear);
    // 注意：示例中的 date2newyear 需要扩展更多年份数据
    let days_3 = date_to_trading_days(year, month, day, &date2newyear);

    format!("{},{},{},{},{},{}", week_count, week_day, days_0, days_1, days_2, days_3)
}
