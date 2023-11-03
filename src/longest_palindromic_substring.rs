#[allow(unused)]
/// # Longest Palindromic Substring
///
/// Give a string `s`, return the longest palindromic substring in `s`
///
/// A string is palindromic if it reads the same forward and backward.
pub fn longest_palindrom(s: String) -> String {
    fn is_palidrome(s: &[u8]) -> bool {
        s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
    }

    for size in (1..=s.len()).rev() {
        let win = s.as_bytes().windows(size);
        win.for_each(|w| {
            println!("for {} -> {:?}", &s, w);
        });
        match s
            .as_bytes()
            .windows(size)
            .find(|substr| is_palidrome(substr))
        {
            Some(pal) => return String::from_utf8(pal.to_vec()).unwrap(),
            None => continue,
        }
    }

    String::from("")
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
