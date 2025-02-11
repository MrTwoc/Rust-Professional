/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters. 
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.
    
    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // TODO: Implement the logic to find the longest substring without repeating characters
    // 0 // Placeholder return value
    // 初始化最长子串的长度为0
    let mut max_length = 0;
    // 初始化滑动窗口的起始位置为0
    let mut start = 0;
    // 初始化一个HashMap，用于存储字符及其在字符串中的最后出现位置
    let mut char_map = std::collections::HashMap::new();

    // 遍历字符串中的每个字符及其索引
    for (end, ch) in s.chars().enumerate() {
        // 如果字符已经在HashMap中存在，说明出现了重复字符
        if let Some(&prev_index) = char_map.get(&ch) {
            // 更新滑动窗口的起始位置为重复字符的下一个位置
            start = std::cmp::max(start, prev_index + 1);
        }
        // 将字符及其索引插入到HashMap中
        char_map.insert(ch, end);
        // 更新最长子串的长度
        max_length = std::cmp::max(max_length, end - start + 1);
    }

    // 返回最长子串的长度
    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1);  // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0);  // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5);  // "abcde"
    }
}
