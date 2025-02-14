/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end], 
    merge all overlapping intervals and return a list of non-overlapping intervals.
    
    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
    
    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement the logic to merge overlapping intervals
    // Vec::new() // Placeholder return value
    // 如果是空区间，返回空
    if intervals.is_empty() {
        return Vec::new();
    }

    // 将不可变参数克隆为可变参数，并按照第一个元素排序
    let mut sort_intervals = intervals.clone();
    sort_intervals.sort_by(|a,b|a[0].cmp(&b[0]));

    // 创建一个可变的结果向量，用于存储合并后的区间
    let mut result = Vec::new();
    // 将第一个区间加入结果向量中
    let mut first = sort_intervals[0].clone();

    // 遍历排序后的区间，合并重叠的区间 .skip(1) 表示跳过第一个元素
    for interval in sort_intervals.iter().skip(1){
        // 如果当前区间的起始点小于等于前一个区间的结束点，说明两个区间有重叠
        if interval[0] <= first[1]{
            // 更新前一个区间的结束点为两个区间结束点的最大值, .max 返回两个值中的最大值
            first[1] = first[1].max(interval[1]);
        } else {
            // 如果当前区间与前一个区间不重叠，将前一个区间加入结果向量中
            result.push(first.clone());
            // 更新前一个区间为当前区间
            first = interval.clone();
        }
    }
    /*
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        第一次循环：interval 是 vec![2, 6]，previous 是 vec![1, 3]。因为 2 <= 3，所以合并区间，previous 变为 vec![1, 6]。
        第二次循环：interval 是 vec![8, 10]，previous 是 vec![1, 6]。因为 8 > 6，所以不重叠，将 vec![1, 6] 添加到 result，previous 更新为 vec![8, 10]。
        第三次循环：interval 是 vec![15, 18]，previous 是 vec![8, 10]。因为 15 > 10，所以不重叠，将 vec![8, 10] 添加到 result，previous 更新为 vec![15, 18]。
     */
    // 将最后一个区间加入结果向量中
    result.push(first);
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18]
        ]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 5]
        ]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![0, 4]
        ]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![1, 10],
            vec![2, 6],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 10]
        ]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![4, 7],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 7],
            vec![8, 10]
        ]);
    }
}
