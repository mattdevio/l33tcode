/// # Longest Palindromic Substring
///
/// Give a string `s`, return the longest palindromic substring in `s`
///
/// A string is palindromic if it reads the same forward and backward.
pub fn longest_palindrom(s: String) -> String {
    todo!()
}

#[cfg(test)]
mod test_5 {
    use super::*;

    #[test]
    fn test_a() {
        // Arrange
        let s = String::from("babad");
        let answers = vec![String::from("bab"), String::from("aba")];
        // Act
        let result = longest_palindrom(s);
        // Assert
        assert!(answers.contains(&result));
    }
    #[test]
    fn test_b() {
        // Arrange
        let s = String::from("cbbd");
        let answers = vec![String::from("bb")];
        // Act
        let result = longest_palindrom(s);
        // Assert
        assert!(answers.contains(&result));
    }
}
