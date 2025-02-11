/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};

/// 检查两个字符串是否是彼此的变位词（anagrams）
///
/// 变位词是指通过重新排列另一个单词或短语的字母而形成的单词或短语，使用所有原始字母恰好一次。
/// 该函数忽略字符串中的空格和标点符号，并将所有字母转换为小写进行比较。
///
/// # 参数
///
/// * `s1` - 第一个字符串
/// * `s2` - 第二个字符串
///
/// # 返回值
///
/// 如果 `s1` 和 `s2` 是变位词，则返回 `true`；否则返回 `false`。
pub fn are_anagrams(s1: String, s2: String) -> bool {
    // 将字符串转换为小写，并过滤掉非字母字符
    let normalized_s1: String = s1
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let normalized_s2: String = s2
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    // 使用 HashMap 来存储每个字符的计数
    let mut char_count = std::collections::HashMap::new();

    // 遍历 normalized_s1 中的每个字符，增加其在 char_count 中的计数
    for c in normalized_s1.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // 遍历 normalized_s2 中的每个字符，减少其在 char_count 中的计数
    for c in normalized_s2.chars() {
        *char_count.entry(c).or_insert(0) -= 1;
    }

    // 检查 char_count 中的所有值是否都为 0
    // 如果所有值都为 0，则表示两个字符串是变位词，返回 true；否则返回 false
    char_count.values().all(|&count| count == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
