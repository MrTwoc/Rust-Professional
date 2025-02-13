/*
    Find Intersection of Two Arrays
    Given two arrays, find the intersection of the arrays and return the elements of the intersection (without duplicates).
    The result should not contain any duplicate elements.

    You need to implement the function `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the elements that are in both arrays.

    Hint: You can solve this problem using sorting, hash sets, or the two-pointer technique.
*/

use std::fmt::{self, Display, Formatter};

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // TODO: Implement the logic to find the intersection of two arrays
    // 这里使用了AI教的双指针技术，本来想用hashset的，但用不了。。
    // 我第一次知道这种 双指针技术
    // Placeholder return value
    // 对 nums1 和 nums2 进行排序
    let mut sorted_nums1 = nums1.clone();
    sorted_nums1.sort_unstable(); // 使用不稳定排序，提高性能
    let mut sorted_nums2 = nums2.clone();
    sorted_nums2.sort_unstable(); // 使用不稳定排序，提高性能

    // 创建一个空的 Vec 用于存储结果
    let mut result = Vec::new();

    // 使用双指针技术找到交集
    let mut i = 0; // 初始化指针 i，指向 sorted_nums1 的第一个元素
    let mut j = 0; // 初始化指针 j，指向 sorted_nums2 的第一个元素
    // 当 i 小于 sorted_nums1 的长度且 j 小于 sorted_nums2 的长度时，继续循环
    while i < sorted_nums1.len() && j < sorted_nums2.len() {
        if sorted_nums1[i] == sorted_nums2[j] {
            // 如果找到相同的元素，将其添加到结果中
            result.push(sorted_nums1[i]);
            // 跳过 sorted_nums1 中重复的元素
            while i < sorted_nums1.len() - 1 && sorted_nums1[i] == sorted_nums1[i + 1] {
                i += 1; // 移动指针 i 到下一个不重复的元素
            }
            // 跳过 sorted_nums2 中重复的元素
            while j < sorted_nums2.len() - 1 && sorted_nums2[j] == sorted_nums2[j + 1] {
                j += 1; // 移动指针 j 到下一个不重复的元素
            }
            // 移动指针 i 和 j 到下一个元素
            i += 1;
            j += 1;
        } else if sorted_nums1[i] < sorted_nums2[j] {
            i += 1; // 如果 sorted_nums1[i] 小于 sorted_nums2[j]，则移动指针 i
        } else {
            j += 1; // 如果 sorted_nums1[i] 大于 sorted_nums2[j]，则移动指针 j
        }
    }

    // 返回结果 Vec
    result
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn test_intersection_3() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![1, 1, 1];
        let nums2 = vec![1, 1, 1];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_intersection_5() {
        let nums1 = vec![10, 20, 30];
        let nums2 = vec![30, 40, 50];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![30]);
    }
}
