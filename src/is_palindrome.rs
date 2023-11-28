#[allow(unused)]
/// # Palindrom Numbers
///
/// Given an integer `x`, return `true` if `x` is a palindrome.
///
/// A integer is palindromic if its reads the same forward and backwards.
pub fn is_palindrome(x: i32) -> bool {
    println!("testing snippet");

    todo!()
}

#[cfg(test)]
mod test_9 {
    use super::*;

    #[test]
    fn test_a() {
        let x = 121;
        let result = is_palindrome(x);
        assert!(result);
    }

    #[test]
    fn test_b() {
        let x = -121;
        let result = is_palindrome(x);
        assert!(!result);
    }

    #[test]
    fn test_c() {
        let x = 10;
        let result = is_palindrome(x);
        assert!(!result);
    }
}
