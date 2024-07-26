// 回文字符串

/// 判断字符串(utf-8)是否为回文字符串
pub fn is_palindrome(str: &str) -> bool {
    // 空字符串不被视为回文字符串
    if str.is_empty() {
        return false;
    }
    // 一个字符的字符串
    if str.len() == 1 {
        return true;
    }

    str.chars().eq(str.chars().rev())
}

/// 判断字符串(utf-8)是否包含回文字符串
pub fn is_contain_palindrome(str: &str) -> bool {
    let chars = str.chars().collect::<Vec<char>>();
    for i in 0..chars.len() {
        for j in (i + 1)..=chars.len() {
            let sub_str = chars[i..j].iter().collect::<String>();
            if sub_str.chars().count() > 1 && is_palindrome(&sub_str) {
                return true;
            }
        }
    }
    false
}

/// 返回字符串(utf-8)中最大的回文字符串
pub fn max_palindrome(str: &str) -> Option<String> {
    if str.len() < 2 {
        return None;
    }

    let mut max_str: String = String::new();
    let chars = str.chars().collect::<Vec<char>>();

    for start in 0..chars.len() {
        for end in (start + 2)..=chars.len() {
            let sub_str = chars[start..end].iter().collect::<String>();
            if is_palindrome(&sub_str) && sub_str.len() > max_str.len() {
                max_str = sub_str;
            }
        }
    }

    if max_str.is_empty() {
        return None;
    }
    Some(max_str)
}

#[cfg(test)]
mod test_palindrome {
    use crate::palindrome::{is_contain_palindrome, is_palindrome, max_palindrome};

    #[test]
    fn test_is_palindrome() {
        assert!(!is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(!is_palindrome("ab"));
        assert!(is_palindrome("aba"));
        assert!(is_palindrome("abcba"));
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("我是人是我"));
        assert!(is_palindrome("上海自来水来自海上"));
    }

    #[test]
    fn test_with_palindrome() {
        assert!(is_contain_palindrome("aba"));
        assert!(!is_contain_palindrome("abc"));
        assert!(!is_contain_palindrome(""));
        assert!(!is_contain_palindrome("a"));
        assert!(is_contain_palindrome("我是人是我"));
        assert!(is_contain_palindrome("上海自来水来自海上"));
    }

    #[test]
    fn test_max_palindrome() {
        assert_eq!(max_palindrome(""), None);
        assert_eq!(max_palindrome("a"), None);
        assert_eq!(max_palindrome("aba"), Some("aba".to_string()));
        assert_eq!(max_palindrome("abcba"), Some("abcba".to_string()));
        assert_eq!(max_palindrome("racecar"), Some("racecar".to_string()));
        assert_eq!(max_palindrome("hello world"), Some("ll".to_string()));
        assert_eq!(
            max_palindrome("madam arora teaches malayalam"),
            Some("malayalam".to_string())
        );
        assert_eq!(max_palindrome("我是人是我"), Some("我是人是我".to_string()));
        assert_eq!(
            max_palindrome("上海自来水来自海上"),
            Some("上海自来水来自海上".to_string())
        );
    }
}
