#[allow(unused)]
/// # Palindrom Numbers
///
/// Given an integer `x`, return `true` if `x` is a palindrome.
///
/// A integer is palindromic if its reads the same forward and backwards.
// Chat GPT explanation:
// In this example, the reverse_number function takes an integer as input and
// uses a while loop to extract the last digit of the number in each iteration,
// appending it to the reversed_number variable.
//     The original number is then updated by removing its last digit
//     (using integer division by 10).
//     This process continues until the original number becomes zero.

pub fn is_palindrome(x: i32) -> bool {
    let mut acc = x;
    let mut y = 0;
    while (acc > 0) {
        y = y * 10 + acc % 10;
        acc /= 10;
    }
    x == y
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
