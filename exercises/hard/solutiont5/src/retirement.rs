use std::cmp::min;
// 借助 AI ，完成以下代码

/* 退休政策参数结构体
   实现原理：
   - base_work_years: 基准工作年限（达到即可退休）
   - start_delay_year: 延迟退休政策起始年份
   - max_delay_months: 最大允许延迟月数
   示例：
   男职工政策 Policy {60, 1965, 36} 表示：
   - 60年工龄可退休
   - 1965年开始实施延迟政策
   - 最多延迟3年（36个月） */
struct Policy {
    base_work_years: i32,
    start_delay_year: i32,
    max_delay_months: i32,
}

/* 核心退休计算函数
   实现步骤：
   1. 解析出生日期
   2. 计算基准退休时间
   3. 判断是否在延迟政策实施前出生
   4. 计算符合政策的月份数
   5. 计算实际延迟月数
   6. 计算总工作月份数
   7. 格式化输出结果
   参数：
   - birth_date: (出生年份, 出生月份)
   - policy: 退休政策参数 */
fn calc(birth_date: (i32, i32), policy: Policy) -> String {
    let (year, month) = birth_date;

    /* 基准退休计算（未受延迟政策影响的情况）
       示例：出生在1960-03，基准工龄60年 → 2020-03退休 */
    fn base_retirement(year: i32, month: i32, work_years: i32) -> String {
        format!("{:04}-{:02},{},0", year + work_years, month, work_years)
    }

    if year < policy.start_delay_year {
        return base_retirement(year, month, policy.base_work_years);
    }

    /* 计算符合政策的月份数
       公式：(出生年份 - 政策起始年)*12 + 出生月份 - 1
       示例：政策起始1970年，出生1975-03：
       (1975-1970)*12 + 3-1 = 62个月 */
    fn eligible_months(birth_year: i32, birth_month: i32, policy_year: i32) -> i32 {
        (birth_year - policy_year) * 12 + birth_month - 1
    }

    let eligible = eligible_months(year, month, policy.start_delay_year);
    let delay = min(eligible / 4 + 1, policy.max_delay_months);
    let total_months = eligible + policy.base_work_years * 12 + delay;

    /* 退休年份计算
       算法：政策起始年 + 总月份数/12（整除）
       示例：政策起始1970年，总月份372 → 1970 + 31 = 2001年 */
    fn retirement_year(policy_year: i32, months: i32) -> i32 {
        policy_year + months / 12
    }

    /* 退休月份计算
       算法：总月份数%12 + 1（月份从1开始）
       示例：总月份372 → 372%12=0 → 0+1=1月 */
    fn retirement_month(months: i32) -> i32 {
        months % 12 + 1
    }

    let ret_year = retirement_year(policy.start_delay_year, total_months);
    let ret_month = retirement_month(total_months);
    let worked_months = total_months - eligible;

    // 格式化工龄
    fn format_age(months: i32) -> String {
        if months % 12 == 0 {
            (months / 12).to_string()
        } else {
            format!("{:.2}", months as f64 / 12.0)
        }
    }

    format!(
        "{:04}-{:02},{},{:.0}",
        ret_year,
        ret_month,
        format_age(worked_months),
        delay
    )
}

/* 公开退休计算接口
   输入参数：
   - time: 出生日期字符串 "YYYY-MM"
   - tp: 人员类型
   处理流程：
   1. 解析日期字符串
   2. 匹配人员类型获取政策参数
   3. 调用核心计算函数 */
pub fn retire_time(time: &str, tp: &str) -> String {
    let mut parts = time.split('-');
    let year = parts.next().unwrap().parse::<i32>().unwrap();
    let month = parts.next().unwrap().parse::<i32>().unwrap();

    /* 预定义政策参数常量
       男职工：60年基准/1965年实施/最多延迟3年
       55岁女职工：55年基准/1970年实施/最多3年
       50岁女职工：50年基准/1975年实施/最多5年 */
    const MALE: Policy = Policy { base_work_years: 60, start_delay_year: 1965, max_delay_months: 36 };
    const FEMALE_55: Policy = Policy { base_work_years: 55, start_delay_year: 1970, max_delay_months: 36 };
    const FEMALE_50: Policy = Policy { base_work_years: 50, start_delay_year: 1975, max_delay_months: 60 };

    match tp {
        "男职工" => calc((year, month), MALE),
        "原法定退休年龄55周岁女职工" => calc((year, month), FEMALE_55),
        "原法定退休年龄50周岁女职工" => calc((year, month), FEMALE_50),
        _ => "未知".to_string(),
    }
}